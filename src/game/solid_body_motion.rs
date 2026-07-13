use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

#[derive(Component)]
pub struct MovingObject;

pub fn update_moving_object(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &Transform), With<MovingObject>>,
) {
    for (mut velocity, transform) in &mut query {
        velocity.x += -2.0 * time.delta_secs() * (transform.translation.x - 0.2);
    }
}
