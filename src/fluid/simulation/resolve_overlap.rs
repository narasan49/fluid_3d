pub mod resolve_overlap_pass;

use bevy::{
    platform::collections::HashMap,
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        sync_world::RenderEntity,
    },
};

use crate::fluid::{
    Fluid3d, GridLength,
    simulation::resolve_overlap::resolve_overlap_pass::ResolveOverlapPassPlugin,
};

pub struct ResolveOverlapPlugin;

impl Plugin for ResolveOverlapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<OverlappedFluids>::default())
            .add_plugins(ResolveOverlapPassPlugin)
            .add_systems(Update, sort_and_sweep);
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub struct OverlappedFluids(pub Vec<Entity>);

enum Marker {
    Beginning,
    End,
}

struct BroadPhaseMarker {
    position: f32,
    entity: Entity,
    render_entity: Entity,
    marker: Marker,
}

impl PartialEq for BroadPhaseMarker {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl PartialOrd for BroadPhaseMarker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

/// 1. オブジェクトのAABB境界をx, y, z軸に射影する。
/// 2. 各軸ですべてのオブジェクトのAABBの境界[bi, ei]を配列に格納
/// 3. 配列をソート
/// 4. 配列を走査して、biならオブジェクトiをactive_listに追加。eiならactive_listから削除
///     - active_listに追加する際のactive_list内のオブジェクトとオブジェクトiは衝突可能性がある。
/// 5. すべての軸で衝突可能性があるなら、AABB同士は衝突する。
fn sort_and_sweep(
    mut commands: Commands,
    query: Query<(Entity, RenderEntity, &Fluid3d, &GlobalTransform)>,
    grid_length: Res<GridLength>,
) {
    let mut markers_x = Vec::with_capacity(10);
    let mut markers_y = Vec::with_capacity(10);
    let mut markers_z = Vec::with_capacity(10);
    for (entity, render_entity, fluid, transform) in &query {
        let half_size = 0.5 * fluid.resolution.as_vec3() * (grid_length.0 as f32);
        let min_position = transform.transform_point(-half_size);
        let max_position = transform.transform_point(half_size);
        markers_x.push(BroadPhaseMarker {
            position: min_position.x,
            entity,
            render_entity,
            marker: Marker::Beginning,
        });
        markers_x.push(BroadPhaseMarker {
            position: max_position.x,
            entity,
            render_entity,
            marker: Marker::End,
        });
        markers_y.push(BroadPhaseMarker {
            position: min_position.y,
            entity,
            render_entity,
            marker: Marker::Beginning,
        });
        markers_y.push(BroadPhaseMarker {
            position: max_position.y,
            entity,
            render_entity,
            marker: Marker::End,
        });
        markers_z.push(BroadPhaseMarker {
            position: min_position.z,
            entity,
            render_entity,
            marker: Marker::Beginning,
        });
        markers_z.push(BroadPhaseMarker {
            position: max_position.z,
            entity,
            render_entity,
            marker: Marker::End,
        });
    }

    markers_x.sort_by(|a, b| {
        return a.partial_cmp(&b).unwrap().then(a.entity.cmp(&b.entity));
    });
    markers_y.sort_by(|a, b| {
        return a.partial_cmp(&b).unwrap().then(a.entity.cmp(&b.entity));
    });
    markers_z.sort_by(|a, b| {
        return a.partial_cmp(&b).unwrap().then(a.entity.cmp(&b.entity));
    });

    let collision_candidates_x = collision_candidates_1d(&markers_x);
    let collision_candidates_y = collision_candidates_1d(&markers_y);
    let collision_candidates_z = collision_candidates_1d(&markers_z);

    for entity in collision_candidates_x.keys() {
        let candidates_x = collision_candidates_x.get(entity).unwrap();
        let Some(candidates_y) = collision_candidates_y.get(entity) else {
            continue;
        };
        let Some(candidates_z) = collision_candidates_z.get(entity) else {
            continue;
        };
        let collided_entities = candidates_x
            .iter()
            .filter_map(|candidate_x| {
                if candidates_y.contains(candidate_x) && candidates_z.contains(candidate_x) {
                    Some(candidate_x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // info!("collision pairs: Entity={entity:?}, Others={collided_entities:?}");
        commands
            .entity(*entity)
            .insert(OverlappedFluids(collided_entities));
    }
}

fn collision_candidates_1d(markers: &Vec<BroadPhaseMarker>) -> HashMap<Entity, Vec<Entity>> {
    let mut active_list = Vec::with_capacity(10);
    let mut collision_candidates = HashMap::new();
    for marker_x in markers {
        match marker_x.marker {
            Marker::Beginning => {
                collision_candidates.insert(marker_x.entity, active_list.clone());
                active_list.push(marker_x.render_entity);
            }
            Marker::End => {
                let (index, _entity) = active_list
                    .iter()
                    .enumerate()
                    .find(|&e| *e.1 == marker_x.render_entity)
                    .unwrap();
                active_list.remove(index);
                if let Some(candidates) = collision_candidates.get_mut(&marker_x.entity) {
                    // ToDo: 重複の可能性はあるが、差し当たって不具合にはならない
                    candidates.extend(active_list.clone());
                }
            }
        }
    }

    collision_candidates
}
