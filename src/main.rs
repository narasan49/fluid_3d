pub mod diagnostics;
mod fluid;
mod game;
mod marching_cubes;
pub mod rigid_body;

use avian3d::{PhysicsPlugins, dynamics::integrator::Gravity};
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin, FreeCameraState},
    dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings},
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};

use crate::{
    diagnostics::fluid_bounds::FluidBoundsPlugin,
    fluid::{
        Fluid3d, Fluid3dPlugin, GridLength, resources::FluidResources,
        simulation::fluid_source::FluidSource,
    },
    game::{
        character_controller::CharacterController,
        input_mode::InputMode,
        scene::{ActiveScene, SceneRoot, demo::spawn_demo_scene, single_fluid::spawn_simple_scene},
        solid_body_motion::update_moving_object,
    },
    marching_cubes::{MarchingCubes, MarchingCubesPlugin},
};

const LENGTH_UNIT: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                    backends: Some(Backends::DX12),
                    ..default()
                })),
                ..default()
            }),
            FreeCameraPlugin,
            InfiniteGridPlugin,
        ))
        .add_plugins(PhysicsPlugins::default().with_length_unit(LENGTH_UNIT))
        .add_plugins((
            Fluid3dPlugin,
            MarchingCubesPlugin,
            FluidBoundsPlugin,
            game::GamePlugin,
        ))
        .add_systems(Startup, setup_persistent_components)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, setup_fluid_render)
        .add_systems(
            Update,
            (
                update_moving_object,
                toggle_fluid_source,
                toggle_free_camera,
                setup_free_camera,
            ),
        )
        .insert_resource(Gravity(9.8 * Vec3::NEG_Y))
        .insert_resource(GridLength(1.0 / LENGTH_UNIT))
        .insert_resource(InputMode::Game)
        .insert_resource(ActiveScene::Demo)
        .run();
}

fn setup_persistent_components(mut commands: Commands) {
    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_length: Res<GridLength>,
    active_scene: Res<ActiveScene>,
) {
    match *active_scene {
        ActiveScene::Demo => {
            spawn_demo_scene(&mut commands, &mut meshes, &mut materials, &grid_length)
        }
        ActiveScene::SingleFluid => spawn_simple_scene(&mut commands),
    }
}

fn setup_free_camera(mut commands: Commands, query: Query<Entity, Added<SceneRoot>>) {
    for entity in &query {
        let mut free_camera_state = FreeCameraState::default();
        free_camera_state.enabled = false;
        commands.entity(entity).with_child((
            Camera3d::default(),
            Camera {
                is_active: false,
                ..default()
            },
            Transform::from_xyz(1.0, 0.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            FreeCamera::default(),
            free_camera_state,
        ));
    }
}

// レベルセットテクスチャをMarchingCubesに渡して描画する
fn setup_fluid_render(
    mut commands: Commands,
    query: Query<(Entity, &FluidResources, &Fluid3d), Added<FluidResources>>,
    grid_length: Res<GridLength>,
) {
    for (entity, resources, fluid) in &query {
        let half_size = 0.5 * fluid.resolution.as_vec3() * (grid_length.0 as f32);
        commands.entity(entity).insert(MarchingCubes {
            grad_sdf: resources.levelset_and_grad_air.clone(),
            resolution: fluid.resolution,
            half_size,
        });
    }
}

fn toggle_fluid_source(mut query: Query<&mut FluidSource>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyF) {
        for mut fluid_source in &mut query {
            fluid_source.active = !fluid_source.active;
        }
    }
}

fn toggle_free_camera(
    mut q_free: Query<(&mut FreeCameraState, &mut Camera), With<FreeCamera>>,
    mut q_player_camera: Query<&mut Camera, Without<FreeCamera>>,
    mut q_player: Query<&mut CharacterController>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        let (mut free_camera_state, mut free_camera) = q_free.single_mut().unwrap();
        free_camera_state.enabled = !free_camera_state.enabled;
        free_camera.is_active = free_camera_state.enabled;
        free_camera.order = if free_camera_state.enabled { 1 } else { 0 };
        let mut player_camera = q_player_camera.single_mut().unwrap();
        player_camera.is_active = !free_camera_state.enabled;
        player_camera.order = if free_camera_state.enabled { 0 } else { 1 };
        for mut character_controller in &mut q_player {
            character_controller.enabled = !free_camera_state.enabled;
        }
    }
}
