use bevy::{
    core_pipeline::{Core3d, Core3dSystems::MainPass, core_3d::CORE_3D_DEPTH_FORMAT},
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedRenderPipelineId, FragmentState, RenderPipelineDescriptor,
        VertexState,
    },
    mesh::{VertexBufferLayout, VertexFormat},
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, BindGroup, BindGroupEntries, BindGroupLayoutEntries, Buffer,
            ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState,
            Face, MultisampleState, PipelineCache, PrimitiveState, RenderPassDescriptor,
            ShaderStages, ShaderType, StencilState, StoreOp, TextureFormat, VertexAttribute,
            VertexStepMode, binding_types::uniform_buffer,
        },
        renderer::{RenderContext, RenderDevice, ViewQuery},
        storage::{GpuShaderBuffer, ShaderBuffer},
        uniform::{ComponentUniforms, DynamicUniformIndex, UniformComponentPlugin},
        view::{ViewDepthTexture, ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms},
    },
};

pub struct MarchingCubesDrawPlugin;

impl Plugin for MarchingCubesDrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<MarchingCubesDrawResource>::default(),
            ExtractComponentPlugin::<MarchingCubesUniform>::default(),
            UniformComponentPlugin::<MarchingCubesUniform>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_systems(
                Render,
                prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            )
            .add_systems(Core3d, marching_cubes_draw.after(MainPass));
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<MarchingCubesDrawPipeline>();
    }
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct MarchingCubesDrawResource {
    #[storage(0, visibility(vertex, fragment))]
    pub vertices: Handle<ShaderBuffer>,
    pub indirect_args: Handle<ShaderBuffer>,
}

#[derive(Component, ExtractComponent, ShaderType, Clone)]
pub struct MarchingCubesUniform {
    pub world_from_local: Mat4,
}

#[derive(Resource)]
struct MarchingCubesDrawPipeline {
    pipeline: CachedRenderPipelineId,
    uniform_bind_group_layout: BindGroupLayoutDescriptor,
    view_bind_group_layout: BindGroupLayoutDescriptor,
}

impl FromWorld for MarchingCubesDrawPipeline {
    fn from_world(world: &mut World) -> Self {
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();
        let uniform_bind_group_layout = BindGroupLayoutDescriptor::new(
            "marching_cubes_uniform_bind_group_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::VERTEX_FRAGMENT,
                uniform_buffer::<MarchingCubesUniform>(true),
            ),
        );

        let view_bind_group_layout = BindGroupLayoutDescriptor::new(
            "marching_cubes_view_bind_group_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::VERTEX_FRAGMENT,
                uniform_buffer::<ViewUniform>(true),
            ),
        );

        let shader = asset_server.load("shaders/marching_cubes/draw.wgsl");

        let pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("marching_cubes_draw_pipeline".into()),
            layout: vec![
                uniform_bind_group_layout.clone(),
                view_bind_group_layout.clone(),
            ],
            vertex: VertexState {
                shader: shader.clone(),
                entry_point: Some("vertex".into()),
                buffers: vec![VertexBufferLayout {
                    array_stride: VertexFormat::Float32x4.size() * 2,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vec![
                        VertexAttribute {
                            format: VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x4,
                            offset: VertexFormat::Float32x4.size(),
                            shader_location: 1,
                        },
                    ],
                }],
                ..default()
            },
            fragment: Some(FragmentState {
                shader,
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                ..default()
            }),
            depth_stencil: Some(DepthStencilState {
                format: CORE_3D_DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(CompareFunction::GreaterEqual),
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            primitive: PrimitiveState {
                cull_mode: Some(Face::Back),
                ..default()
            },
            zero_initialize_workgroup_memory: true,
            multisample: MultisampleState {
                count: 4,
                ..default()
            },
            ..default()
        });

        Self {
            pipeline,
            uniform_bind_group_layout,
            view_bind_group_layout,
        }
    }
}

#[derive(Component)]
struct MarchingCubesDrawBindings {
    uniform_bind_group: BindGroup,
    uniform_index: u32,
    view_bind_group: BindGroup,
    vertex_buffer: Buffer,
    indirect_buffer: Buffer,
}

fn prepare_bind_groups(
    mut commands: Commands,
    query: Query<(
        Entity,
        &MarchingCubesDrawResource,
        &DynamicUniformIndex<MarchingCubesUniform>,
    )>,
    pipeline: Res<MarchingCubesDrawPipeline>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    view_uniforms: Res<ViewUniforms>,
    mc_uniforms: Res<ComponentUniforms<MarchingCubesUniform>>,
    buffers: Res<RenderAssets<GpuShaderBuffer>>,
) {
    for (entity, resource, uniform_index) in &query {
        let (Some(mc_uniform), Some(view_uniform)) =
            (mc_uniforms.binding(), view_uniforms.uniforms.binding())
        else {
            continue;
        };

        let uniform_bind_group = render_device.create_bind_group(
            "marching_cubes_uniform_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipeline.uniform_bind_group_layout),
            &BindGroupEntries::single(mc_uniform),
        );

        let view_bind_group = render_device.create_bind_group(
            "marching_cubes_view_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipeline.view_bind_group_layout),
            &BindGroupEntries::single(view_uniform),
        );

        let vertex_buffer = buffers.get(&resource.vertices).unwrap().buffer.clone();
        let indirect_buffer = buffers.get(&resource.indirect_args).unwrap().buffer.clone();

        commands.entity(entity).insert(MarchingCubesDrawBindings {
            uniform_bind_group,
            uniform_index: uniform_index.index(),
            view_bind_group,
            vertex_buffer,
            indirect_buffer,
        });
    }
}

fn marching_cubes_draw(
    mut render_context: RenderContext,
    view: ViewQuery<(&ViewTarget, &ViewDepthTexture, &ViewUniformOffset)>,
    query: Query<&MarchingCubesDrawBindings>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<MarchingCubesDrawPipeline>,
) {
    let (target, depth, view_offset) = view.into_inner();
    let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline.pipeline) else {
        let pipeline_state = pipeline_cache.get_render_pipeline_state(pipeline.pipeline);
        info!("{:?}", pipeline_state);
        return;
    };

    for bindings in &query {
        info_once!("[once] draw marching cubes mesh");
        let mut pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("marching_cubes_draw"),
            color_attachments: &[Some(target.get_color_attachment())],
            depth_stencil_attachment: Some(depth.get_attachment(StoreOp::Store)),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        pass.set_render_pipeline(pipeline);
        pass.set_vertex_buffer(0, bindings.vertex_buffer.slice(..));
        pass.set_bind_group(0, &bindings.uniform_bind_group, &[bindings.uniform_index]);
        pass.set_bind_group(1, &bindings.view_bind_group, &[view_offset.offset]);
        pass.draw_indirect(&bindings.indirect_buffer, 0);
    }
}
