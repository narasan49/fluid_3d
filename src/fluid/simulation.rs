pub mod initialize;

use bevy::{
    core_pipeline::schedule::camera_driver,
    ecs::query::QueryData,
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        render_resource::{ComputePassDescriptor, PipelineCache},
        renderer::RenderContext,
    },
};

use crate::fluid::{
    Fluid3d,
    compute_pass::FluidComputePassPlugin,
    pipeline::FluidPipeline,
    simulation::initialize::{InitializeBindGroup, InitializePass, InitializePipeline},
    workgroup::WORKGROUP_SIZE,
};

pub struct FluidSimulationPlugin;

impl Plugin for FluidSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FluidComputePassPlugin::<InitializePass>::default());

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
    init_bind_group: &'static InitializeBindGroup,
}

fn update_simulation_state(
    init_pipeline: Res<InitializePipeline>,
    pipeline_cache: Res<PipelineCache>,
    mut state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::Loading => {
            if init_pipeline.is_ready(&pipeline_cache) {
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
    query: Query<(&Fluid3d, SimulationBindGroups)>,
    pipeline_cache: Res<PipelineCache>,
    init_pipeline: Res<InitializePipeline>,
    state: ResMut<SimulationState>,
) {
    match *state {
        SimulationState::Loading => {}
        SimulationState::Init => {
            let mut pass =
                render_context
                    .command_encoder()
                    .begin_compute_pass(&ComputePassDescriptor {
                        label: Some("init_fluid_3d"),
                        ..default()
                    });

            for (fluid, bind_groups) in &query {
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
        SimulationState::Update => {}
    }
}
