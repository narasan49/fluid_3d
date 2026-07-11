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

pub struct SolveVelocityPass;

impl FluidComputePass for SolveVelocityPass {
    type B = SolveVelocityBindGroup;
    type P = SolveVelocityPipeline;
    type R = SolveVelocityResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct SolveVelocityResource {
    #[storage_texture(0, image_format = R16Float, dimension = "3d", access = ReadWrite)]
    pub u_mac: Handle<Image>,
    #[storage_texture(1, image_format = R16Float, dimension = "3d", access = ReadWrite)]
    pub v_mac: Handle<Image>,
    #[storage_texture(2, image_format = R16Float, dimension = "3d", access = ReadWrite)]
    pub w_mac: Handle<Image>,
    #[storage_texture(3, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub p: Handle<Image>,
    #[storage_texture(4, image_format = Rgba16Float, dimension = "3d", access = ReadOnly)]
    pub fluid_fraction: Handle<Image>,
    #[storage_texture(5, image_format = Rgba16Float, dimension = "3d", access = ReadOnly)]
    pub u_solid: Handle<Image>,
    #[storage_texture(6, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_air0: Handle<Image>,
}

impl SolveVelocityResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            u_mac: resources.u_mac.clone(),
            v_mac: resources.v_mac.clone(),
            w_mac: resources.w_mac.clone(),
            p: resources.p.clone(),
            fluid_fraction: resources.fluid_fraction.clone(),
            u_solid: resources.u_solid.clone(),
            levelset_air0: resources.levelset_air0.clone(),
        }
    }
}

#[derive(Resource)]
pub struct SolveVelocityPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl SolveVelocityPipeline {
    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_group: &SolveVelocityBindGroup,
        uniform_bind_group: &FluidUniformBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("solve_velocity");
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

impl FluidPipeline for SolveVelocityPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for SolveVelocityPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout = SolveVelocityResource::bind_group_layout_descriptor(render_device);
        let uniform_bind_group_layout = &world.resource::<FluidUniformBindGroupLayout>().0;

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("solve_velocity_pipeline".into()),
            layout: vec![bind_group_layout.clone(), uniform_bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/solve_velocity.wgsl"),
            entry_point: Some("solve_velocity".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct SolveVelocityBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for SolveVelocityBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}
