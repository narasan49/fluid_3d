use bevy::{
    asset::{embedded_asset, load_embedded_asset},
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_resource::{AsBindGroup, BindGroup, PipelineCache},
        renderer::RenderDevice,
    },
};

use crate::fluid::{compute_pass::FluidComputePass, pipeline::FluidPipeline};

pub struct InitializePass;

impl FluidComputePass for InitializePass {
    type B = InitializeBindGroup;
    type P = InitializePipeline;
    type R = InitializeResource;

    fn register_assets(app: &mut App) {
        embedded_asset!(app, "initialize.wgsl");
    }
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct InitializeResource {
    #[storage_texture(0, image_format = R32Float, dimension = "3d", access = WriteOnly)]
    pub levelset_air0: Handle<Image>,
    // #[storage_texture(1, image_format = R32Float, dimension = "3d", access = WriteOnly)]
    // pub levelset_air1: Handle<Image>,
    // #[storage_texture(2, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    // pub u0: Handle<Image>,
}

#[derive(Resource)]
pub struct InitializePipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl FluidPipeline for InitializePipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }
}

impl FromWorld for InitializePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let bind_group_layout = InitializeResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("initialize_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: load_embedded_asset!(world, "initilize.wgsl"),
            entry_point: Some("initilize".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct InitializeBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for InitializeBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
