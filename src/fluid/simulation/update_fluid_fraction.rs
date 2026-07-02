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

pub struct UpdateFluidFractionPass;

impl FluidComputePass for UpdateFluidFractionPass {
    type B = UpdateFluidFractionBindGroup;
    type P = UpdateFluidFractionPipeline;
    type R = UpdateFluidFractionResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct UpdateFluidFractionResource {
    #[storage_texture(0, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_solid: Handle<Image>,
    #[storage_texture(1, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    pub fluid_fraction: Handle<Image>,
}

impl UpdateFluidFractionResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            levelset_solid: resources.levelset_solid.clone(),
            fluid_fraction: resources.fluid_fraction.clone(),
        }
    }
}

#[derive(Resource)]
pub struct UpdateFluidFractionPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl UpdateFluidFractionPipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &UpdateFluidFractionBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("update_fluid_fraction");
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let num_wg = num_workgroups(resolution + UVec3::ONE, workgroup_size);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.dispatch_workgroups(num_wg.x, num_wg.y, num_wg.z);

        pass.pop_debug_group();
    }
}

impl FluidPipeline for UpdateFluidFractionPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for UpdateFluidFractionPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            UpdateFluidFractionResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("update_fluid_fraction_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/update_fluid_fraction.wgsl"),
            entry_point: Some("update_fluid_fraction".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct UpdateFluidFractionBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for UpdateFluidFractionBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
