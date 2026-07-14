pub mod advect_levelset;
pub mod advect_velocity;
pub mod apply_forces;
pub mod divergence;
pub mod extrapolate_velocity;
pub mod fluid_uniform;
pub mod grid_transition;
pub mod initialize;
pub mod projection;
pub mod reinitialize_levelset;
pub mod solid_to_fluid;
pub mod solve_velocity;
pub mod update_fluid_fraction;
pub mod update_levelset_grad;
pub mod update_solid;

use bevy::{
    core_pipeline::schedule::camera_driver,
    ecs::{query::QueryData, system::SystemParam},
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
        extrapolate_velocity::{
            ExtrapolateVelocityBindGroups, ExtrapolateVelocityPipeline, ExtrapolateVelocityPlugin,
        },
        fluid_uniform::{FluidUniformBindGroup, FluidUniformPlugin},
        grid_transition::{
            CollocatedToMacBindGroup, CollocatedToMacPass, CollocatedToMacPipeline,
            MacToCollocatedBindGroup, MacToCollocatedPass, MacToCollocatedPipeline,
        },
        initialize::{InitializeBindGroup, InitializePass, InitializePipeline},
        projection::{
            MultigridIterationGonfig, MultigridNumLevels, MultigridProjectionBindGroups,
            MultigridProjectionPassPlugin, MultigridProjectionPipeline,
        },
        reinitialize_levelset::{
            FastIterativeMethodInitializeActiveLabelsPipeline,
            FastIterativeMethodInitializePipeline, FastIterativeMethodUpdatePipeline,
            ReinitializeLevelSetBindGroups, ReinitializeLevelSetPlugin,
            reinitialize_levelset_dispatch,
        },
        solid_to_fluid::{SolidBodyBufferBindGroup, SolidToFluidPlugin},
        solve_velocity::{SolveVelocityBindGroup, SolveVelocityPass, SolveVelocityPipeline},
        update_fluid_fraction::{
            UpdateFluidFractionBindGroup, UpdateFluidFractionPass, UpdateFluidFractionPipeline,
        },
        update_levelset_grad::{
            UpdateLevelSetGradBindGroup, UpdateLevelSetGradPass, UpdateLevelSetGradPipeline,
        },
        update_solid::{UpdateSolidBindGroup, UpdateSolidPass, UpdateSolidPipeline},
    },
    workgroup::WORKGROUP_SIZE,
};

pub struct FluidSimulationPlugin;

impl Plugin for FluidSimulationPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "simulation/area_fraction.wgsl");
        load_shader_library!(app, "simulation/primitive_sdf.wgsl");

        app.add_plugins((FluidUniformPlugin, SolidToFluidPlugin))
            .add_plugins((
                FluidComputePassPlugin::<InitializePass>::default(),
                FluidComputePassPlugin::<UpdateSolidPass>::default(),
                FluidComputePassPlugin::<UpdateFluidFractionPass>::default(),
                FluidComputePassPlugin::<AdvectVelocityPass>::default(),
                FluidComputePassPlugin::<ApplyForcesPass>::default(),
                FluidComputePassPlugin::<CollocatedToMacPass>::default(),
                FluidComputePassPlugin::<DivergencePass>::default(),
                MultigridProjectionPassPlugin,
                FluidComputePassPlugin::<SolveVelocityPass>::default(),
                FluidComputePassPlugin::<MacToCollocatedPass>::default(),
                FluidComputePassPlugin::<AdvectLevelSetPass>::default(),
                ReinitializeLevelSetPlugin,
                FluidComputePassPlugin::<UpdateLevelSetGradPass>::default(),
                ExtrapolateVelocityPlugin,
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
    collocated_to_mac_bind_group: &'static CollocatedToMacBindGroup,
    divergence_bind_group: &'static DivergenceBindGroup,
    multigrid_projection_bind_groups: &'static MultigridProjectionBindGroups,
    solve_velocity_bind_group: &'static SolveVelocityBindGroup,
    mac_to_collocated_bind_group: &'static MacToCollocatedBindGroup,
    advect_levelset_bind_group: &'static AdvectLevelSetBindGroup,
    reinitialize_levelset_bind_groups: ReinitializeLevelSetBindGroups,
    update_grad_levelset_bind_group: &'static UpdateLevelSetGradBindGroup,
    extrapolate_velocity_bind_groups: &'static ExtrapolateVelocityBindGroups,
}

#[derive(SystemParam)]
struct FluidPipelines<'w> {
    init_pipeline: Res<'w, InitializePipeline>,
    update_solid_pipeline: Res<'w, UpdateSolidPipeline>,
    update_fluid_fraction_pipeline: Res<'w, UpdateFluidFractionPipeline>,
    advect_velocity_pipeline: Res<'w, AdvectVelocityPipeline>,
    apply_forces_pipeline: Res<'w, ApplyForcesPipeline>,
    collocated_to_mac_pipeline: Res<'w, CollocatedToMacPipeline>,
    divergence_pipeline: Res<'w, DivergencePipeline>,
    multigrid_projection_pipeline: Res<'w, MultigridProjectionPipeline>,
    solve_velocity_pipeline: Res<'w, SolveVelocityPipeline>,
    mac_to_collocated_pipeline: Res<'w, MacToCollocatedPipeline>,
    advect_levelset_pipeline: Res<'w, AdvectLevelSetPipeline>,
    fim_init_pipeline: Res<'w, FastIterativeMethodInitializePipeline>,
    fim_init_labels_pipeline: Res<'w, FastIterativeMethodInitializeActiveLabelsPipeline>,
    fim_update_pipeline: Res<'w, FastIterativeMethodUpdatePipeline>,
    update_grad_levelset_pipeline: Res<'w, UpdateLevelSetGradPipeline>,
    extrapolate_velocity_pipeline: Res<'w, ExtrapolateVelocityPipeline>,
}

fn update_simulation_state(
    pipelines: FluidPipelines,
    pipeline_cache: Res<PipelineCache>,
    mut state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::Loading => {
            if pipelines.init_pipeline.is_ready(&pipeline_cache)
                && pipelines.advect_velocity_pipeline.is_ready(&pipeline_cache)
                && pipelines.apply_forces_pipeline.is_ready(&pipeline_cache)
                && pipelines
                    .collocated_to_mac_pipeline
                    .is_ready(&pipeline_cache)
                && pipelines.divergence_pipeline.is_ready(&pipeline_cache)
                && pipelines.update_solid_pipeline.is_ready(&pipeline_cache)
                && pipelines
                    .update_fluid_fraction_pipeline
                    .is_ready(&pipeline_cache)
                && pipelines
                    .multigrid_projection_pipeline
                    .is_ready(&pipeline_cache)
                && pipelines.solve_velocity_pipeline.is_ready(&pipeline_cache)
                && pipelines
                    .mac_to_collocated_pipeline
                    .is_ready(&pipeline_cache)
                && pipelines.advect_levelset_pipeline.is_ready(&pipeline_cache)
                && pipelines.fim_init_pipeline.is_ready(&pipeline_cache)
                && pipelines.fim_init_labels_pipeline.is_ready(&pipeline_cache)
                && pipelines.fim_update_pipeline.is_ready(&pipeline_cache)
                && pipelines
                    .update_grad_levelset_pipeline
                    .is_ready(&pipeline_cache)
                && pipelines
                    .extrapolate_velocity_pipeline
                    .is_ready(&pipeline_cache)
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
    solid_body_bind_group: Res<SolidBodyBufferBindGroup>,
    pipeline_cache: Res<PipelineCache>,
    pipelines: FluidPipelines,
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
                pipelines.init_pipeline.dispatch(
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
                pipelines.update_solid_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.update_solid_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    &solid_body_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.update_fluid_fraction_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.update_fluid_fraction_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.advect_velocity_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.advect_velocity_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.apply_forces_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.apply_forces_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.collocated_to_mac_pipeline.dispatch(
                    &mut pass,
                    &pipeline_cache,
                    &bind_groups.collocated_to_mac_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.divergence_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.divergence_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.multigrid_projection_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.multigrid_projection_bind_groups,
                    &bind_groups.fluid_uniform_bind_group,
                    multigrid_config,
                    multigrid_levels.0,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.solve_velocity_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.solve_velocity_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.mac_to_collocated_pipeline.dispatch(
                    &mut pass,
                    &pipeline_cache,
                    &bind_groups.mac_to_collocated_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.extrapolate_velocity_pipeline.dispatch(
                    &mut pass,
                    &pipeline_cache,
                    bind_groups.extrapolate_velocity_bind_groups,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.advect_levelset_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.advect_levelset_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                reinitialize_levelset_dispatch(
                    &pipelines.fim_init_pipeline,
                    &pipelines.fim_init_labels_pipeline,
                    &pipelines.fim_update_pipeline,
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.reinitialize_levelset_bind_groups,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );

                pipelines.update_grad_levelset_pipeline.dispatch(
                    &pipeline_cache,
                    &mut pass,
                    &bind_groups.update_grad_levelset_bind_group,
                    &bind_groups.fluid_uniform_bind_group,
                    fluid.resolution,
                    WORKGROUP_SIZE,
                );
            }
        }
    }
}
