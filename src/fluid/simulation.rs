pub mod initialize;

use bevy::prelude::*;

use crate::fluid::{compute_pass::FluidComputePassPlugin, simulation::initialize::InitializePass};

pub struct FluidSimulationPlugin;

impl Plugin for FluidSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FluidComputePassPlugin::<InitializePass>::default());
    }
}
