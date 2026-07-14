use bevy::{
    core_pipeline::{Core3d, Core3dSystems::MainPass},
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

use crate::marching_cubes::MarchingCubes;

pub struct BuildVertexBufferPlugin;

impl Plugin for BuildVertexBufferPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<BuildVertexBufferResource>::default());
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(
                Render,
                prepare_bind_group.in_set(RenderSystems::PrepareBindGroups),
            )
            .add_systems(Core3d, build_vertex_buffer.before(MainPass));
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<BuildVertexBufferPipeline>();
    }
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct BuildVertexBufferResource {
    #[storage(0, visibility(compute))]
    pub vertices: Handle<ShaderBuffer>,
    #[storage(1, visibility(compute))]
    pub indirect_args: Handle<ShaderBuffer>,
    #[storage_texture(2, image_format = Rgba16Float, access = ReadOnly, dimension = "3d")]
    pub grad_sdf: Handle<Image>,
    #[storage(3, read_only, visibility(compute))]
    pub lookup_table: Handle<ShaderBuffer>,
}

#[derive(Resource)]
struct BuildVertexBufferPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl FromWorld for BuildVertexBufferPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            BuildVertexBufferResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("build_vertex_buffer_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/marching_cubes/build_vertex_buffer.wgsl"),
            entry_point: Some("build_vertex_buffer".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
struct BuildVertexBufferBindGroup {
    bind_group: BindGroup,
}

fn prepare_bind_group<'a>(
    mut commands: Commands,
    query: Query<(Entity, &BuildVertexBufferResource)>,
    pipeline: Res<BuildVertexBufferPipeline>,
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
            .insert(BuildVertexBufferBindGroup { bind_group });
    }
}

fn build_vertex_buffer(
    mut render_context: RenderContext,
    query: Query<(
        &MarchingCubes,
        &BuildVertexBufferBindGroup,
        &BuildVertexBufferResource,
    )>,
    pipeline: Res<BuildVertexBufferPipeline>,
    pipeline_cache: Res<PipelineCache>,
    buffers: Res<RenderAssets<GpuShaderBuffer>>,
) {
    let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) else {
        return;
    };

    for (marching_cubes, bind_group, resource) in &query {
        let Some(indirect_args) = buffers.get(&resource.indirect_args) else {
            continue;
        };
        info_once!("[once] build vertex buffer");

        // vertex countをリセット
        render_context
            .command_encoder()
            .clear_buffer(&indirect_args.buffer, 0, Some(4u64));

        let mut pass =
            render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor {
                    label: Some("build_vertex_buffer"),
                    ..default()
                });

        let workgroup_size = UVec3::splat(8);
        let num_workgroups =
            (marching_cubes.resolution - UVec3::ONE + workgroup_size - UVec3::ONE) / workgroup_size;
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.dispatch_workgroups(num_workgroups.x, num_workgroups.y, num_workgroups.z);
    }
}
