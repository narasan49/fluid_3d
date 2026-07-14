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

pub struct ApplyForcesPass;

impl FluidComputePass for ApplyForcesPass {
    type B = ApplyForcesBindGroup;
    type P = ApplyForcesPipeline;
    type R = ApplyForcesResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct ApplyForcesResource {
    #[storage_texture(0, image_format = Rgba16Float, dimension = "3d", access = ReadWrite)]
    pub u1: Handle<Image>,
    #[storage_texture(1, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_air0: Handle<Image>,
    #[storage_texture(2, image_format = Rgba16Float, dimension = "3d", access = ReadOnly)]
    pub non_solid_fraction: Handle<Image>,
}

impl ApplyForcesResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            u1: resources.u1.clone(),
            levelset_air0: resources.levelset_air0.clone(),
            non_solid_fraction: resources.non_solid_fraction.clone(),
        }
    }
}

#[derive(Resource)]
pub struct ApplyForcesPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl ApplyForcesPipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &ApplyForcesBindGroup,
        uniform_bind_group: &FluidUniformBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("apply_forces");
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

impl FluidPipeline for ApplyForcesPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for ApplyForcesPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout = ApplyForcesResource::bind_group_layout_descriptor(render_device);
        let uniform_bind_group_layout = &world.resource::<FluidUniformBindGroupLayout>().0;

        let pipeline: CachedComputePipelineId =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("apply_forces_pipeline".into()),
                layout: vec![bind_group_layout.clone(), uniform_bind_group_layout.clone()],
                shader: asset_server.load("shaders/simulation/apply_forces.wgsl"),
                entry_point: Some("apply_forces".into()),
                ..default()
            });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct ApplyForcesBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for ApplyForcesBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
