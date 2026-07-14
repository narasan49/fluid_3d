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

pub struct UpdateAreaFractionsPass;

impl FluidComputePass for UpdateAreaFractionsPass {
    type B = UpdateAreaFractionsBindGroup;
    type P = UpdateAreaFractionsPipeline;
    type R = UpdateAreaFractionsResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct UpdateAreaFractionsResource {
    #[storage_texture(0, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_solid: Handle<Image>,
    #[storage_texture(1, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_air0: Handle<Image>,
    #[storage_texture(2, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    pub non_solid_fraction: Handle<Image>,
    #[storage_texture(3, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    pub non_fluid_fraction: Handle<Image>,
}

impl UpdateAreaFractionsResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            levelset_solid: resources.levelset_solid.clone(),
            levelset_air0: resources.levelset_air0.clone(),
            non_solid_fraction: resources.non_solid_fraction.clone(),
            non_fluid_fraction: resources.non_fluid_fraction.clone(),
        }
    }
}

#[derive(Resource)]
pub struct UpdateAreaFractionsPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl UpdateAreaFractionsPipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &UpdateAreaFractionsBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("update_area_fraction");
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let num_wg = num_workgroups(resolution + UVec3::ONE, workgroup_size);
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.dispatch_workgroups(num_wg.x, num_wg.y, num_wg.z);

        pass.pop_debug_group();
    }
}

impl FluidPipeline for UpdateAreaFractionsPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for UpdateAreaFractionsPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            UpdateAreaFractionsResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("update_area_fractions_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/update_area_fractions.wgsl"),
            entry_point: Some("update_area_fractions".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct UpdateAreaFractionsBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for UpdateAreaFractionsBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
