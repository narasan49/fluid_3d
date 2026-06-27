use bevy::{
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_resource::{AsBindGroup, BindGroup, ComputePass, PipelineCache},
        renderer::RenderDevice,
    },
};

use crate::fluid::{
    compute_pass::FluidComputePass,
    pipeline::{FluidPipeline, is_pipeline_loaded},
    resources::FluidResources,
    workgroup::num_workgroups,
};

pub struct InitializePass;

impl FluidComputePass for InitializePass {
    type B = InitializeBindGroup;
    type P = InitializePipeline;
    type R = InitializeResource;
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

impl InitializeResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            levelset_air0: resources.levelset_air0.clone(),
        }
    }
}

#[derive(Resource)]
pub struct InitializePipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl InitializePipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &InitializeBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("initialize");
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let num_wg = num_workgroups(resolution, workgroup_size);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.dispatch_workgroups(num_wg.x, num_wg.y, num_wg.z);

        pass.pop_debug_group();
    }
}

impl FluidPipeline for InitializePipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for InitializePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout = InitializeResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("initialize_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/initialize.wgsl"),
            entry_point: Some("initialize".into()),
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
