use bevy::prelude::*;

use crate::fluid::simulation::fluid_source::FluidSource;

#[derive(Component)]
pub struct AutoStopFluidSource(pub f32);

pub fn update_auto_stop_fluid_source(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FluidSource, &mut AutoStopFluidSource)>,
    time: Res<Time>,
) {
    for (entity, mut source, mut auto_stop) in &mut query {
        if source.active {
            auto_stop.0 -= time.delta_secs();
            if auto_stop.0 <= 0.0 {
                source.active = false;
                commands.entity(entity).remove::<AutoStopFluidSource>();
            }
        }
    }
}
