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

pub struct MacToCollocatedPass;

impl FluidComputePass for MacToCollocatedPass {
    type B = MacToCollocatedBindGroup;
    type P = MacToCollocatedPipeline;
    type R = MacToCollocatedResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct MacToCollocatedResource {
    #[storage_texture(0, image_format = R16Float, dimension = "3d", access = ReadOnly)]
    pub u_mac: Handle<Image>,
    #[storage_texture(1, image_format = R16Float, dimension = "3d", access = ReadOnly)]
    pub v_mac: Handle<Image>,
    #[storage_texture(2, image_format = R16Float, dimension = "3d", access = ReadOnly)]
    pub w_mac: Handle<Image>,
    #[storage_texture(3, image_format = Rgba16Float, dimension = "3d", access = WriteOnly)]
    pub u0: Handle<Image>,
    #[storage_texture(4, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_air0: Handle<Image>,
}

impl MacToCollocatedResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            u_mac: resources.u_mac.clone(),
            v_mac: resources.v_mac.clone(),
            w_mac: resources.w_mac.clone(),
            u0: resources.u0.clone(),
            levelset_air0: resources.levelset_air0.clone(),
        }
    }
}

#[derive(Resource)]
pub struct MacToCollocatedPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl MacToCollocatedPipeline {
    pub fn dispatch(
        &self,
        pass: &mut ComputePass,
        pipeline_cache: &PipelineCache,
        bind_group: &MacToCollocatedBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let workgroups = num_workgroups(resolution, workgroup_size);
        pass.push_debug_group("mac_to_collocated");
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        pass.pop_debug_group();
    }
}

impl FromWorld for MacToCollocatedPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            MacToCollocatedResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("mac_to_collocated_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/grid_transition/mac_to_collocated.wgsl"),
            entry_point: Some("mac_to_collocated".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl FluidPipeline for MacToCollocatedPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

#[derive(Component)]
pub struct MacToCollocatedBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for MacToCollocatedBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}

pub struct CollocatedToMacPass;

impl FluidComputePass for CollocatedToMacPass {
    type B = CollocatedToMacBindGroup;
    type P = CollocatedToMacPipeline;
    type R = CollocatedToMacResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct CollocatedToMacResource {
    #[storage_texture(0, image_format = R16Float, dimension = "3d", access = WriteOnly)]
    pub u_mac: Handle<Image>,
    #[storage_texture(1, image_format = R16Float, dimension = "3d", access = WriteOnly)]
    pub v_mac: Handle<Image>,
    #[storage_texture(2, image_format = R16Float, dimension = "3d", access = WriteOnly)]
    pub w_mac: Handle<Image>,
    #[storage_texture(3, image_format = Rgba16Float, dimension = "3d", access = ReadOnly)]
    pub u1: Handle<Image>,
}

impl CollocatedToMacResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            u_mac: resources.u_mac.clone(),
            v_mac: resources.v_mac.clone(),
            w_mac: resources.w_mac.clone(),
            u1: resources.u1.clone(),
        }
    }
}

#[derive(Resource)]
pub struct CollocatedToMacPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl CollocatedToMacPipeline {
    pub fn dispatch(
        &self,
        pass: &mut ComputePass,
        pipeline_cache: &PipelineCache,
        bind_group: &CollocatedToMacBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();
        let workgroups = num_workgroups(resolution - UVec3::ONE, workgroup_size);
        pass.push_debug_group("collocated_to_mac");
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        pass.pop_debug_group();
    }
}

impl FromWorld for CollocatedToMacPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            CollocatedToMacResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("collocated_to_mac_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/grid_transition/collocated_to_mac.wgsl"),
            entry_point: Some("collocated_to_mac".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl FluidPipeline for CollocatedToMacPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

#[derive(Component)]
pub struct CollocatedToMacBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for CollocatedToMacBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
