use bevy::{
    prelude::*,
    render::extract_component::{ExtractComponent, ExtractComponentPlugin},
};

use crate::fluid::{
    resources::FluidResources,
    simulation::{FluidSimulationPlugin, initialize::InitializeResource},
};

pub mod compute_pass;
pub mod pipeline;
pub mod resources;
pub mod simulation;
pub mod workgroup;

pub struct Fluid3dPlugin;

impl Plugin for Fluid3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FluidSimulationPlugin)
            .add_plugins(ExtractComponentPlugin::<Fluid3d>::default())
            .add_systems(Update, setup_fluid_component);
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub struct Fluid3d {
    pub resolution: UVec3,
    pub rho: f32,
    pub gravity: Vec3,
}

fn setup_fluid_component(
    mut commands: Commands,
    query: Query<(Entity, &Fluid3d), Added<Fluid3d>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (entity, fluid3d) in &query {
        let resources = FluidResources::new(&mut images, fluid3d.resolution);

        let init_resource = InitializeResource::new(&resources);

        commands.entity(entity).insert((resources, init_resource));
    }
}
