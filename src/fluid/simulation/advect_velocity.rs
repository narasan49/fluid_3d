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
    simulation::fluid_uniform::{FluidUniformBindGroup, FluidUniformBindGroupLayout},
    workgroup::num_workgroups,
};

pub struct AdvectVelocityPass;

impl FluidComputePass for AdvectVelocityPass {
    type B = AdvectVelocityBindGroup;
    type P = AdvectVelocityPipeline;
    type R = AdvectVelocityResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct AdvectVelocityResource {
    #[storage_texture(0, image_format = Rgba16Float, dimension = "3d", access = ReadOnly)]
    pub u0: Handle<Image>,
    #[storage_texture(1, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    pub u1: Handle<Image>,
}

impl AdvectVelocityResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            u0: resources.u0.clone(),
            u1: resources.u1.clone(),
        }
    }
}

#[derive(Resource)]
pub struct AdvectVelocityPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl AdvectVelocityPipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &AdvectVelocityBindGroup,
        uniform_bind_group: &FluidUniformBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("advect_velocity");
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let num_wg = num_workgroups(resolution, workgroup_size);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.set_bind_group(
            1,
            &uniform_bind_group.bind_group,
            &[uniform_bind_group.index],
        );
        pass.dispatch_workgroups(num_wg.x, num_wg.y, num_wg.z);

        pass.pop_debug_group();
    }
}

impl FluidPipeline for AdvectVelocityPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for AdvectVelocityPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout = AdvectVelocityResource::bind_group_layout_descriptor(render_device);
        let uniform_bind_group_layout = &world.resource::<FluidUniformBindGroupLayout>().0;

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("advect_velocity_pipeline".into()),
            layout: vec![bind_group_layout.clone(), uniform_bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/advect_velocity.wgsl"),
            entry_point: Some("advect_velocity".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct AdvectVelocityBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for AdvectVelocityBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
