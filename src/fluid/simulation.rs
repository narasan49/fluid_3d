pub mod advect_levelset;
pub mod advect_velocity;
pub mod apply_forces;
pub mod divergence;
pub mod fluid_uniform;
pub mod initialize;
pub mod projection;
pub mod solve_velocity;
pub mod update_fluid_fraction;
pub mod update_solid;

use bevy::{
    core_pipeline::schedule::camera_driver,
    ecs::query::QueryData,
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        render_resource::{ComputePassDescriptor, PipelineCache},
        renderer::RenderContext,
    },
    shader::load_shader_library,
};

use crate::fluid::{
    Fluid3d,
    compute_pass::FluidComputePassPlugin,
    pipeline::FluidPipeline,
    simulation::{
        advect_levelset::{AdvectLevelSetBindGroup, AdvectLevelSetPass, AdvectLevelSetPipeline},
        advect_velocity::{AdvectVelocityBindGroup, AdvectVelocityPass, AdvectVelocityPipeline},
        apply_forces::{ApplyForcesBindGroup, ApplyForcesPass, ApplyForcesPipeline},
        divergence::{DivergenceBindGroup, DivergencePass, DivergencePipeline},
        fluid_uniform::{FluidUniformBindGroup, FluidUniformPlugin},
        initialize::{InitializeBindGroup, InitializePass, InitializePipeline},
        projection::{
            MultigridIterationGonfig, MultigridNumLevels, MultigridProjectionBindGroups,
            MultigridProjectionPassPlugin, MultigridProjectionPipeline,
        },
        solve_velocity::{SolveVelocityBindGroup, SolveVelocityPass, SolveVelocityPipeline},
        update_fluid_fraction::{
            UpdateFluidFractionBindGroup, UpdateFluidFractionPass, UpdateFluidFractionPipeline,
        },
        update_solid::{UpdateSolidBindGroup, UpdateSolidPass, UpdateSolidPipeline},
    },
    workgroup::WORKGROUP_SIZE,
};

pub struct FluidSimulationPlugin;

impl Plugin for FluidSimulationPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "simulation/area_fraction.wgsl");

        app.add_plugins((
            FluidUniformPlugin,
            FluidComputePassPlugin::<InitializePass>::default(),
            FluidComputePassPlugin::<UpdateSolidPass>::default(),
            FluidComputePassPlugin::<UpdateFluidFractionPass>::default(),
            FluidComputePassPlugin::<AdvectVelocityPass>::default(),
            FluidComputePassPlugin::<ApplyForcesPass>::default(),
            FluidComputePassPlugin::<DivergencePass>::default(),
            MultigridProjectionPassPlugin,
            FluidComputePassPlugin::<SolveVelocityPass>::default(),
            FluidComputePassPlugin::<AdvectLevelSetPass>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<SimulationState>()
            .add_systems(
                Render,
                update_simulation_state.in_set(RenderSystems::Prepare),
            )
            .add_systems(RenderGraph, run_simulation.before(camera_driver));
    }
}

#[derive(Resource, Default)]
enum SimulationState {
    #[default]
    Loading,
    Init,
    Update,
}

#[derive(QueryData)]
struct SimulationBindGroups {
    fluid_uniform_bind_group: &'static FluidUniformBindGroup,
    init_bind_group: &'static InitializeBindGroup,
    update_solid_bind_group: &'static UpdateSolidBindGroup,
    update_fluid_fraction_bind_group: &'static UpdateFluidFractionBindGroup,
    advect_velocity_bind_group: &'static AdvectVelocityBindGroup,
    apply_forces_bind_group: &'static ApplyForcesBindGroup,
    divergence_bind_group: &'static DivergenceBindGroup,
    multigrid_projection_bind_groups: &'static MultigridProjectionBindGroups,
    solve_velocity_bind_group: &'static SolveVelocityBindGroup,
    advect_levelset_bind_group: &'static AdvectLevelSetBindGroup,
}

fn update_simulation_state(
    init_pipeline: Res<InitializePipeline>,
    update_solid_pipeline: Res<UpdateSolidPipeline>,
    update_fluid_fraction_pipeline: Res<UpdateFluidFractionPipeline>,
    advect_velocity_pipeline: Res<AdvectVelocityPipeline>,
    apply_forces_pipeline: Res<ApplyForcesPipeline>,
    divergence_pipeline: Res<DivergencePipeline>,
    multigrid_projection_pipeline: Res<MultigridProjectionPipeline>,
    solve_velocity_pipeline: Res<SolveVelocityPipeline>,
    advect_levelset_pipeline: Res<AdvectLevelSetPipeline>,
    pipeline_cache: Res<PipelineCache>,
    mut state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::Loading => {
            if init_pipeline.is_ready(&pipeline_cache)
                && advect_velocity_pipeline.is_ready(&pipeline_cache)
                && apply_forces_pipeline.is_ready(&pipeline_cache)
                && divergence_pipeline.is_ready(&pipeline_cache)
                && update_solid_pipeline.is_ready(&pipeline_cache)
                && update_fluid_fraction_pipeline.is_ready(&pipeline_cache)
                && multigrid_projection_pipeline.is_ready(&pipeline_cache)
                && solve_velocity_pipeline.is_ready(&pipeline_cache)
                && advect_levelset_pipeline.is_ready(&pipeline_cache)
            {
                *state = SimulationState::Init;
            }
        }
        SimulationState::Init => {
            *state = SimulationState::Update;
        }
        SimulationState::Update => {}
    }
}

fn run_simulation(
    mut render_context: RenderContext,
    query: Query<(
        &Fluid3d,
        SimulationBindGroups,
        &MultigridIterationGonfig,
        &MultigridNumLevels,
    )>,
    pipeline_cache: Res<PipelineCache>,
    init_pipeline: Res<InitializePipeline>,
    update_solid_pipeline: Res<UpdateSolidPipeline>,
    update_fluid_fraction_pipeline: Res<UpdateFluidFractionPipeline>,
    advect_velocity_pipeline: Res<AdvectVelocityPipeline>,
    apply_forces_pipeline: Res<ApplyForcesPipeline>,
    divergence_pipeline: Res<DivergencePipeline>,
    projection_pipeline: Res<MultigridProjectionPipeline>,
    solve_velocity_pipeline: Res<SolveVelocityPipeline>,
    advect_levelset_pipeline: Res<AdvectLevelSetPipeline>,
    state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::Loading => {}
        SimulationState::Init => {
            for (fluid, bind_groups, _, _) in &query {
                let mut pass =
                    render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor {
                            label: Some("init_fluid_3d"),
                            ..default()
                        });
                info_once!("[once] initializing fluid");
                init_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    bind_groups.init_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );
            }
        }
        SimulationState::Update => {
            for (fluid, bind_groups, multigrid_config, multigrid_levels) in &query {
                let mut pass =
                    render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor {
                            label: Some("run_fluid_3d"),
                            ..default()
                        });

                info_once!("[once] running fluid simulation");
                update_solid_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.update_solid_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                update_fluid_fraction_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.update_fluid_fraction_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                advect_velocity_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.advect_velocity_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                apply_forces_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.apply_forces_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                divergence_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.divergence_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                projection_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.multigrid_projection_bind_groups,
                    &bind_groups.fluid_uniform_bind_group,
                    multigrid_config,
                    multigrid_levels.0,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                solve_velocity_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.solve_velocity_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                advect_levelset_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.advect_levelset_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );
            }
        }
    }
}
