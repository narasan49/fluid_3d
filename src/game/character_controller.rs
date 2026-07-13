use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_character_input, velocity_damping));
    }
}

const CHARACTER_ACCELERATION: f32 = 10.0;
const DAMPING_RATE: f32 = 0.8;

#[derive(Component)]
pub struct CharacterController;

fn handle_character_input(
    time: Res<Time>,
    mut query: Query<&mut LinearVelocity, With<CharacterController>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec3::ZERO;
    if input.any_pressed([KeyCode::ArrowUp]) {
        direction -= Vec3::Z;
    }
    if input.any_pressed([KeyCode::ArrowDown]) {
        direction += Vec3::Z;
    }
    if input.any_pressed([KeyCode::ArrowLeft]) {
        direction -= Vec3::X;
    }
    if input.any_pressed([KeyCode::ArrowRight]) {
        direction += Vec3::X;
    }
    if direction == Vec3::ZERO {
        return;
    }
    for mut linear_velocity in &mut query {
        let delta = CHARACTER_ACCELERATION * direction.normalize() * time.delta_secs();
        linear_velocity.0 += delta;
    }
}

fn velocity_damping(mut query: Query<&mut LinearVelocity, With<CharacterController>>) {
    for mut linear_velocity in &mut query {
        linear_velocity.x *= DAMPING_RATE;
        linear_velocity.z *= DAMPING_RATE;
    }
}
