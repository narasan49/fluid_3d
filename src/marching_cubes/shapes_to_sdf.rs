pub mod update_shapes_buffer;

use bevy::{
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, BindGroup, ComputePassDescriptor, PipelineCache},
        renderer::{RenderContext, RenderDevice},
        storage::{GpuShaderBuffer, ShaderBuffer},
        texture::{FallbackImage, GpuImage},
    },
};

use crate::{fluid::workgroup::num_workgroups, marching_cubes::MarchingCubes};

pub struct ShapesToSdfPlugin;

impl Plugin for ShapesToSdfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<ShapesToSdfResource>::default())
            .add_systems(Update, update_shapes_buffer::update_shapes_buffer);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(
            Render,
            prepare_bind_group.in_set(RenderSystems::PrepareBindGroups),
        );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<ShapesToSdfPipeline>();
    }
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct ShapesToSdfResource {
    #[storage_texture(0, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    pub grad_sdf: Handle<Image>,
    #[storage(1, visibility(compute))]
    pub shapes: Handle<ShaderBuffer>,
}

#[derive(Resource)]
pub struct ShapesToSdfPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl FromWorld for ShapesToSdfPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout = ShapesToSdfResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("shapes_to_sdf_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/marching_cubes/shapes_to_sdf.wgsl"),
            entry_point: Some("shapes_to_sdf".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct ShapesToSdfBindGroup {
    bind_group: BindGroup,
}

fn prepare_bind_group<'a>(
    mut commands: Commands,
    query: Query<(Entity, &ShapesToSdfResource)>,
    pipeline: Res<ShapesToSdfPipeline>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    mut param: (
        Res<'a, RenderAssets<GpuImage>>,
        Res<'a, FallbackImage>,
        Res<'a, RenderAssets<GpuShaderBuffer>>,
    ),
) {
    for (entity, resource) in &query {
        let bind_group = resource
            .as_bind_group(
                &pipeline.bind_group_layout,
                &render_device,
                &pipeline_cache,
                &mut param,
            )
            .unwrap()
            .bind_group;

        commands
            .entity(entity)
            .insert(ShapesToSdfBindGroup { bind_group });
    }
}

pub fn run_shapes_to_sdf_pass(
    mut render_context: RenderContext,
    query: Query<(&MarchingCubes, &ShapesToSdfBindGroup)>,
    pipeline: Res<ShapesToSdfPipeline>,
    pipeline_cache: Res<PipelineCache>,
) {
    let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) else {
        return;
    };

    for (marcing_cubes, bind_group) in &query {
        let mut pass =
            render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: Some("shapes_to_sdf"),
                    ..default()
                });

        let workgroup_size = UVec3::splat(8);
        let workgroups = num_workgroups(marcing_cubes.resolution, workgroup_size);

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
    }
}
