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
    simulation::{
        fluid_source::fluid_sources_uniform::{
            FluidSourcesBindGroupLayout, FluidSourcesUniformBindGroup,
        },
        fluid_uniform::{FluidUniformBindGroup, FluidUniformBindGroupLayout},
    },
    workgroup::num_workgroups,
};

pub struct UpdateFluidSourcesPass;

impl FluidComputePass for UpdateFluidSourcesPass {
    type B = UpdateFluidSourcesBindGroup;
    type P = UpdateFluidSourcesPipeline;
    type R = UpdateFluidSourcesResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct UpdateFluidSourcesResource {
    #[storage_texture(0, image_format = R32Float, dimension = "3d", access = ReadWrite)]
    pub levelset_air0: Handle<Image>,
    #[storage_texture(1, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    pub u0: Handle<Image>,
}

impl UpdateFluidSourcesResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            levelset_air0: resources.levelset_air0.clone(),
            u0: resources.u0.clone(),
        }
    }
}

#[derive(Resource)]
pub struct UpdateFluidSourcesPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl UpdateFluidSourcesPipeline {
    pub fn dispatch(
        &self,
        pass: &mut ComputePass,
        pipeline_cache: &PipelineCache,
        bind_group: &UpdateFluidSourcesBindGroup,
        uniform_bind_group: &FluidUniformBindGroup,
        fluid_sources_uniform_bind_group: &FluidSourcesUniformBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("update_fluid_sources");
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let workgroups = num_workgroups(resolution, workgroup_size);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.set_bind_group(
            1,
            &uniform_bind_group.bind_group,
            &[uniform_bind_group.index],
        );
        pass.set_bind_group(
            2,
            &fluid_sources_uniform_bind_group.bind_group,
            &[fluid_sources_uniform_bind_group.index],
        );
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        pass.pop_debug_group();
    }
}

impl FromWorld for UpdateFluidSourcesPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            UpdateFluidSourcesResource::bind_group_layout_descriptor(render_device);
        let fluid_uniform_bind_group_layout = world.resource::<FluidUniformBindGroupLayout>();
        let fluid_sources_bind_group_layout = world.resource::<FluidSourcesBindGroupLayout>();

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("update_fluid_sources_pipeline".into()),
            layout: vec![
                bind_group_layout.clone(),
                fluid_uniform_bind_group_layout.0.clone(),
                fluid_sources_bind_group_layout.bind_group_layout.clone(),
            ],
            shader: asset_server.load("shaders/simulation/fluid_source/update_fluid_sources.wgsl"),
            entry_point: Some("update_fluid_sources".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl FluidPipeline for UpdateFluidSourcesPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

#[derive(Component)]
pub struct UpdateFluidSourcesBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for UpdateFluidSourcesBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
