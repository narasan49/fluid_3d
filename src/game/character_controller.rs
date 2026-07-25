use avian3d::{
    collision::collider::Collider,
    dynamics::{integrator::Gravity, rigid_body::LinearVelocity},
    spatial_query::{ShapeCastConfig, SpatialQuery, SpatialQueryFilter},
};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_grounded,
                apply_gravity,
                handle_character_input,
                velocity_damping,
            )
                .chain(),
        )
        .add_systems(Update, (update_player_camera, respawn));
    }
}

const CHARACTER_ACCELERATION: f32 = 20.0;
const DAMPING_RATE: f32 = 0.8;
const JUMP_SPEED: f32 = 2.0;
const LOWER_BOUNDARY: f32 = -3.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterController {
    pub enabled: bool,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Grounded;

fn handle_character_input(
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &mut LinearVelocity,
        &CharacterController,
        Has<Grounded>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec3::ZERO;
    if input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction -= Vec3::Z;
    }
    if input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction += Vec3::Z;
    }
    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction -= Vec3::X;
    }
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction += Vec3::X;
    }
    if direction == Vec3::ZERO {
        return;
    }
    for (transform, mut linear_velocity, controller, grounded) in &mut query {
        if !controller.enabled {
            continue;
        }
        let quat = transform.rotation;
        let delta =
            CHARACTER_ACCELERATION * quat.mul_vec3(direction).normalize() * time.delta_secs();
        linear_velocity.0 += delta;

        // Jump
        if input.all_pressed([KeyCode::Space]) {
            if grounded {
                linear_velocity.y += JUMP_SPEED;
            }
        }
    }
}

fn velocity_damping(mut query: Query<&mut LinearVelocity, With<CharacterController>>) {
    for mut linear_velocity in &mut query {
        linear_velocity.x *= DAMPING_RATE;
        linear_velocity.z *= DAMPING_RATE;
    }
}

fn update_grounded(
    mut commands: Commands,
    query: Query<(Entity, &Collider, &GlobalTransform), With<CharacterController>>,
    spatial_query: SpatialQuery,
) {
    for (entity, collider, transform) in &query {
        let Some(capsule) = collider.shape().as_capsule() else {
            continue;
        };

        let hit = spatial_query.cast_shape(
            collider,
            transform.translation(),
            transform.rotation(),
            Dir3::NEG_Y,
            &ShapeCastConfig::from_max_distance(capsule.half_height()),
            &SpatialQueryFilter::from_excluded_entities([entity]),
        );

        let grounded = hit.is_some();

        if grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn apply_gravity(
    mut query: Query<(&mut LinearVelocity, Has<Grounded>), With<CharacterController>>,
    gravity: Res<Gravity>,
    time: Res<Time>,
) {
    for (mut velocity, is_grounded) in &mut query {
        if is_grounded {
            velocity.y = 0.0;
        } else {
            velocity.0 += gravity.0 * time.delta_secs();
        }
    }
}

fn update_player_camera(
    mut query: Query<&mut Transform, With<Player>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let delta = accumulated_mouse_motion.delta;

    for mut transform in &mut query {
        transform.rotate_local_y(-delta.x * time.delta_secs() * 0.1);
    }
}

fn respawn(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        if transform.translation.y < LOWER_BOUNDARY {
            transform.translation = Vec3::new(1.0, 5.0, 0.0);
        }
    }
}
