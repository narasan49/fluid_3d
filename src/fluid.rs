use bevy::{
    prelude::*,
    render::extract_component::{ExtractComponent, ExtractComponentPlugin},
};

use crate::fluid::{
    resources::FluidResources,
    simulation::{
        FluidSimulationPlugin, advect_velocity::AdvectVelocityResource,
        apply_forces::ApplyForcesResource, divergence::DivergenceResource,
        fluid_uniform::FluidUniform, initialize::InitializeResource,
        projection::setup_multigrid_resources, update_fluid_fraction::UpdateFluidFractionResource,
        update_solid::UpdateSolidResource,
    },
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
#[require(Transform)]
pub struct Fluid3d {
    pub resolution: UVec3,
    pub rho: f32,
    pub gravity: Vec3,
}

fn setup_fluid_component(
    mut commands: Commands,
    query: Query<(Entity, &Fluid3d, &Transform), Added<Fluid3d>>,
    mut images: ResMut<Assets<Image>>,
) {
    for (entity, fluid3d, transform) in &query {
        let resources = FluidResources::new(&mut images, fluid3d.resolution);
        let fluid_uniform = FluidUniform {
            dx: 1.0,
            dt: 0.0,
            rho: fluid3d.rho,
            resolution: fluid3d.resolution,
            gravity: fluid3d.gravity,
            transform: transform.to_matrix(),
        };
        let init_resource = InitializeResource::new(&resources);

        let update_solid_resource = UpdateSolidResource::new(&resources);
        let update_fluid_fraction_resource = UpdateFluidFractionResource::new(&resources);
        let advect_velocity_resource = AdvectVelocityResource::new(&resources);
        let apply_forces_resource = ApplyForcesResource::new(&resources);
        let divergence_resource = DivergenceResource::new(&resources);

        setup_multigrid_resources(
            &mut commands,
            &mut images,
            entity,
            &resources,
            fluid3d.resolution,
        );

        commands.entity(entity).insert((
            fluid_uniform,
            resources,
            init_resource,
            update_solid_resource,
            update_fluid_fraction_resource,
            advect_velocity_resource,
            apply_forces_resource,
            divergence_resource,
        ));
    }
}
