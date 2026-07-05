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

pub struct UpdateGradLevelSetPass;

impl FluidComputePass for UpdateGradLevelSetPass {
    type B = UpdateGradLevelSetBindGroup;
    type P = UpdateGradLevelSetPipeline;
    type R = UpdateGradLevelSetResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct UpdateGradLevelSetResource {
    #[storage_texture(0, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_air0: Handle<Image>,
    #[storage_texture(1, image_format = Rgba16Snorm, dimension = "3d", access = WriteOnly)]
    pub grad_levelset_air: Handle<Image>,
}

impl UpdateGradLevelSetResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            levelset_air0: resources.levelset_air0.clone(),
            grad_levelset_air: resources.grad_levelset_air.clone(),
        }
    }
}

#[derive(Resource)]
pub struct UpdateGradLevelSetPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl UpdateGradLevelSetPipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &UpdateGradLevelSetBindGroup,
        uniform_bind_group: &FluidUniformBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("update_grad_levelset");
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

impl FluidPipeline for UpdateGradLevelSetPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for UpdateGradLevelSetPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            UpdateGradLevelSetResource::bind_group_layout_descriptor(render_device);
        let uniform_bind_group_layout = &world.resource::<FluidUniformBindGroupLayout>().0;

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("update_grad_levelset_pipeline".into()),
            layout: vec![bind_group_layout.clone(), uniform_bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/update_grad_levelset.wgsl"),
            entry_point: Some("update_grad_levelset".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct UpdateGradLevelSetBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for UpdateGradLevelSetBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
