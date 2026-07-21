use bevy::{
    prelude::*,
    render::{
        MainWorld, RenderApp,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        sync_world::RenderEntity,
    },
};

use crate::fluid::{
    resources::FluidResources,
    simulation::{
        FluidSimulationPlugin,
        advect_levelset::AdvectLevelSetResource,
        advect_velocity::AdvectVelocityResource,
        apply_forces::ApplyForcesResource,
        divergence::DivergenceResource,
        extrapolate_velocity::ExtrapolateVelocityResource,
        fluid_source::{
            fluid_sources_uniform::FluidSourcesUniform,
            update_fluid_sources::UpdateFluidSourcesResource,
        },
        fluid_uniform::FluidUniform,
        grid_transition::{CollocatedToMacResource, MacToCollocatedResource},
        initialize::InitializeResource,
        projection::setup_multigrid_resources,
        reinitialize_levelset::{
            FastIterativeMethodInitializeActiveLabelsResource,
            FastIterativeMethodInitializeResource, FastIterativeMethodUpdateResource,
        },
        solve_velocity::SolveVelocityResource,
        update_area_fractions::UpdateAreaFractionsResource,
        update_levelset_grad::UpdateLevelSetGradResource,
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
            .add_plugins((
                ExtractComponentPlugin::<Fluid3d>::default(),
                ExtractComponentPlugin::<FluidResources>::default(),
                ExtractComponentPlugin::<BoundaryConditions>::default(),
            ))
            .add_systems(Update, setup_fluid_component);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(ExtractSchedule, extract_fluid_status);
    }
}

#[derive(Component, ExtractComponent, Clone)]
#[require(Transform, FluidSourcesUniform, BoundaryConditions, FluidStatus)]
pub struct Fluid3d {
    pub resolution: UVec3,
    pub rho: f32,
    pub gravity: Vec3,
}

#[derive(Component, ExtractComponent, Clone, Default)]
pub struct BoundaryConditions {
    pub x_min: FluidBoundaryMethod,
    pub y_min: FluidBoundaryMethod,
    pub z_min: FluidBoundaryMethod,
    pub x_max: FluidBoundaryMethod,
    pub y_max: FluidBoundaryMethod,
    pub z_max: FluidBoundaryMethod,
}

impl BoundaryConditions {
    fn conditions_min(&self) -> UVec3 {
        UVec3 {
            x: self.x_min.to_u32(),
            y: self.y_min.to_u32(),
            z: self.z_min.to_u32(),
        }
    }
    fn conditions_max(&self) -> UVec3 {
        UVec3 {
            x: self.x_max.to_u32(),
            y: self.y_max.to_u32(),
            z: self.z_max.to_u32(),
        }
    }
}

#[derive(Clone, Default)]
pub enum FluidBoundaryMethod {
    #[default]
    Wall,
    Open,
}

impl FluidBoundaryMethod {
    fn to_u32(&self) -> u32 {
        match self {
            FluidBoundaryMethod::Wall => 0u32,
            FluidBoundaryMethod::Open => 1u32,
        }
    }
}

#[derive(Component, Default)]
pub enum FluidStatus {
    #[default]
    RequestReset,
    Running,
}

#[derive(Component)]
pub enum FluidStatusRenderWorld {
    Reset,
    Uninitialized,
    Initialized,
}

fn extract_fluid_status(mut commands: Commands, mut main_world: ResMut<MainWorld>) {
    let mut fluid_status_query = main_world.query::<(RenderEntity, &mut FluidStatus)>();

    for (render_entity, mut fluid_status) in fluid_status_query.iter_mut(&mut main_world) {
        match *fluid_status {
            FluidStatus::RequestReset => {
                commands
                    .entity(render_entity)
                    .insert(FluidStatusRenderWorld::Reset);
                *fluid_status = FluidStatus::Running;
            }
            FluidStatus::Running => {}
        }
    }
}

#[derive(Resource)]
pub struct GridLength(pub f32);

fn setup_fluid_component(
    mut commands: Commands,
    query: Query<(Entity, &Fluid3d, &BoundaryConditions, &Transform), Added<Fluid3d>>,
    grid_length: Res<GridLength>,
    mut images: ResMut<Assets<Image>>,
) {
    for (entity, fluid3d, boundary_conditions, transform) in &query {
        let resources = FluidResources::new(&mut images, fluid3d.resolution);
        let half_size = 0.5 * fluid3d.resolution.as_vec3() * (grid_length.0 as f32);
        let fluid_uniform = FluidUniform {
            dx: grid_length.0,
            dt: 0.0,
            rho: fluid3d.rho,
            resolution: fluid3d.resolution,
            gravity: fluid3d.gravity,
            transform: transform.to_matrix(),
            half_size,
            boundary_condition_min: boundary_conditions.conditions_min(),
            boundary_condition_max: boundary_conditions.conditions_max(),
        };
        let init_resource = InitializeResource::new(&resources);

        let update_solid_resource = UpdateSolidResource::new(&resources);
        let update_fluid_sources_resource = UpdateFluidSourcesResource::new(&resources);
        let update_fluid_fraction_resource = UpdateAreaFractionsResource::new(&resources);
        let advect_velocity_resource = AdvectVelocityResource::new(&resources);
        let apply_forces_resource = ApplyForcesResource::new(&resources);
        let collocated_to_mac_resource = CollocatedToMacResource::new(&resources);
        let divergence_resource = DivergenceResource::new(&resources);

        setup_multigrid_resources(
            &mut commands,
            &mut images,
            entity,
            &resources,
            fluid3d.resolution,
        );

        let solve_velocity_resource = SolveVelocityResource::new(&resources);
        let mac_to_collocated_resource = MacToCollocatedResource::new(&resources);
        let advect_levelset_resource = AdvectLevelSetResource::new(&resources);

        let fim_init_resource = FastIterativeMethodInitializeResource::new(&resources);
        let fim_init_labels_resource =
            FastIterativeMethodInitializeActiveLabelsResource::new(&resources);
        let fim_update_resource = FastIterativeMethodUpdateResource::new(&resources);
        let update_grad_levelset_resource = UpdateLevelSetGradResource::new(&resources);
        let extrapolate_velocity_resource = ExtrapolateVelocityResource::new(&resources);

        commands
            .entity(entity)
            .insert((fluid_uniform, resources, init_resource))
            .insert((
                update_solid_resource,
                update_fluid_sources_resource,
                update_fluid_fraction_resource,
                advect_velocity_resource,
                apply_forces_resource,
                collocated_to_mac_resource,
                divergence_resource,
                solve_velocity_resource,
                mac_to_collocated_resource,
                advect_levelset_resource,
                fim_init_resource,
                fim_init_labels_resource,
                fim_update_resource,
                update_grad_levelset_resource,
                extrapolate_velocity_resource,
            ));
    }
}
