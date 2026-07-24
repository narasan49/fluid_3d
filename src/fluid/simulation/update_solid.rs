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
        fluid_uniform::{FluidUniformBindGroup, FluidUniformBindGroupLayout},
        solid_to_fluid::{SolidBodyBufferBindGroup, SolidBodyBufferBindGroupLayout},
    },
    workgroup::num_workgroups,
};

pub struct UpdateSolidAndApronPass;

impl FluidComputePass for UpdateSolidAndApronPass {
    type B = UpdateSolidAndApronBindGroup;
    type P = UpdateSolidAndApronPipeline;
    type R = UpdateSolidAndApronResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct UpdateSolidAndApronResource {
    #[storage_texture(0, image_format = Rgba16Float, dimension = "3d", access = ReadWrite)]
    pub u_solid: Handle<Image>,
    #[storage_texture(1, image_format = R32Float, dimension = "3d", access = WriteOnly)]
    pub levelset_solid: Handle<Image>,
    #[storage_texture(2, image_format = R32Float, dimension = "3d", access = WriteOnly)]
    pub levelset_air0: Handle<Image>,
}

impl UpdateSolidAndApronResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            u_solid: resources.u_solid.clone(),
            levelset_solid: resources.levelset_solid.clone(),
            levelset_air0: resources.levelset_air0.clone(),
        }
    }
}

#[derive(Resource)]
pub struct UpdateSolidAndApronPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl UpdateSolidAndApronPipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &UpdateSolidAndApronBindGroup,
        uniform_bind_group: &FluidUniformBindGroup,
        solid_body_bind_group: &SolidBodyBufferBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("update_solid_and_apron");
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let num_wg = num_workgroups(resolution, workgroup_size);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.set_bind_group(
            1,
            &uniform_bind_group.bind_group,
            &[uniform_bind_group.index],
        );
        pass.set_bind_group(2, &solid_body_bind_group.0, &[]);
        pass.dispatch_workgroups(num_wg.x, num_wg.y, num_wg.z);

        pass.pop_debug_group();
    }
}

impl FluidPipeline for UpdateSolidAndApronPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for UpdateSolidAndApronPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            UpdateSolidAndApronResource::bind_group_layout_descriptor(render_device);
        let uniform_bind_group_layout = world.resource::<FluidUniformBindGroupLayout>();
        let solid_body_bind_group_layout = world.resource::<SolidBodyBufferBindGroupLayout>();

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("update_solid_and_apron_pipeline".into()),
            layout: vec![
                bind_group_layout.clone(),
                uniform_bind_group_layout.0.clone(),
                solid_body_bind_group_layout.0.clone(),
            ],
            shader: asset_server.load("shaders/simulation/update_solid_and_apron.wgsl"),
            entry_point: Some("update_solid_and_apron".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct UpdateSolidAndApronBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for UpdateSolidAndApronBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
