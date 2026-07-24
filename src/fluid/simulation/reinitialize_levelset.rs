use bevy::{
    ecs::query::QueryData,
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
    compute_pass::{FluidComputePass, FluidComputePassPlugin},
    pipeline::{FluidPipeline, is_pipeline_loaded},
    resources::FluidResources,
    workgroup::num_workgroups,
};

pub struct ReinitializeLevelSetPlugin;

impl Plugin for ReinitializeLevelSetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FluidComputePassPlugin::<FastIterativeMethodInitializePass>::default(),
            FluidComputePassPlugin::<FastIterativeMethodInitializeActiveLabelsPass>::default(),
            FluidComputePassPlugin::<FastIterativeMethodUpdatePass>::default(),
        ));
    }
}

pub struct FastIterativeMethodInitializePass;

impl FluidComputePass for FastIterativeMethodInitializePass {
    type B = FastIterativeMethodInitializeBindGroup;
    type P = FastIterativeMethodInitializePipeline;
    type R = FastIterativeMethodInitializeResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct FastIterativeMethodInitializeResource {
    #[storage_texture(0, image_format = R32Float, dimension = "3d", access = ReadOnly)]
    pub levelset_air1: Handle<Image>,
    #[storage_texture(1, image_format = R32Float, dimension = "3d", access = WriteOnly)]
    pub levelset_air0: Handle<Image>,
    #[storage_texture(2, image_format = R8Uint, dimension = "3d", access = WriteOnly)]
    pub labels0: Handle<Image>,
}

impl FastIterativeMethodInitializeResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            levelset_air1: resources.levelset_air1.clone(),
            levelset_air0: resources.levelset_air0.clone(),
            labels0: resources.labels0.clone(),
        }
    }
}

#[derive(Resource)]
pub struct FastIterativeMethodInitializePipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl FromWorld for FastIterativeMethodInitializePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            FastIterativeMethodInitializeResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("fast_iterative_method_initialize_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/reinitialize_levelset/initialize.wgsl"),
            entry_point: Some("initialize".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl FluidPipeline for FastIterativeMethodInitializePipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

#[derive(Component)]
pub struct FastIterativeMethodInitializeBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for FastIterativeMethodInitializeBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}

pub struct FastIterativeMethodInitializeActiveLabelsPass;

impl FluidComputePass for FastIterativeMethodInitializeActiveLabelsPass {
    type B = FastIterativeMethodInitializeActiveLabelsBindGroup;
    type P = FastIterativeMethodInitializeActiveLabelsPipeline;
    type R = FastIterativeMethodInitializeActiveLabelsResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct FastIterativeMethodInitializeActiveLabelsResource {
    #[storage_texture(0, image_format = R8Uint, dimension = "3d", access = ReadOnly)]
    pub labels0: Handle<Image>,
    #[storage_texture(1, image_format = R8Uint, dimension = "3d", access = WriteOnly)]
    pub labels1: Handle<Image>,
}

impl FastIterativeMethodInitializeActiveLabelsResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            labels0: resources.labels0.clone(),
            labels1: resources.labels1.clone(),
        }
    }
}

#[derive(Resource)]
pub struct FastIterativeMethodInitializeActiveLabelsPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl FromWorld for FastIterativeMethodInitializeActiveLabelsPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            FastIterativeMethodInitializeActiveLabelsResource::bind_group_layout_descriptor(
                render_device,
            );

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("fast_iterative_method_initialize_active_labels_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server
                .load("shaders/simulation/reinitialize_levelset/initialize_active_labels.wgsl"),
            entry_point: Some("initialize_active_labels".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl FluidPipeline for FastIterativeMethodInitializeActiveLabelsPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

#[derive(Component)]
pub struct FastIterativeMethodInitializeActiveLabelsBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for FastIterativeMethodInitializeActiveLabelsBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}

pub struct FastIterativeMethodUpdatePass;

impl FluidComputePass for FastIterativeMethodUpdatePass {
    type B = FastIterativeMethodUpdateBindGroup;
    type P = FastIterativeMethodUpdatePipeline;
    type R = FastIterativeMethodUpdateResource;
}

#[derive(Component, ExtractComponent, Clone, AsBindGroup)]
pub struct FastIterativeMethodUpdateResource {
    #[storage_texture(0, image_format = R8Uint, dimension = "3d", access = ReadWrite)]
    pub labels1: Handle<Image>,
    #[storage_texture(1, image_format = R32Float, dimension = "3d", access = ReadWrite)]
    pub levelset_air0: Handle<Image>,
}

impl FastIterativeMethodUpdateResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            labels1: resources.labels1.clone(),
            levelset_air0: resources.levelset_air0.clone(),
        }
    }
}

#[derive(Resource)]
pub struct FastIterativeMethodUpdatePipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl FromWorld for FastIterativeMethodUpdatePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout =
            FastIterativeMethodUpdateResource::bind_group_layout_descriptor(render_device);

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("fast_iterative_method_update_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/reinitialize_levelset/update.wgsl"),
            entry_point: Some("update".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

impl FluidPipeline for FastIterativeMethodUpdatePipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

#[derive(Component)]
pub struct FastIterativeMethodUpdateBindGroup {
    bind_group: BindGroup,
}

impl From<BindGroup> for FastIterativeMethodUpdateBindGroup {
    fn from(bind_group: BindGroup) -> Self {
        Self { bind_group }
    }
}

#[derive(QueryData)]
pub struct ReinitializeLevelSetBindGroups {
    initialize_bind_group: &'static FastIterativeMethodInitializeBindGroup,
    initialize_active_labels_bind_group:
        &'static FastIterativeMethodInitializeActiveLabelsBindGroup,
    update_bind_group: &'static FastIterativeMethodUpdateBindGroup,
}

pub fn reinitialize_levelset_dispatch(
    init_pipeline: &FastIterativeMethodInitializePipeline,
    init_labels_pipeline: &FastIterativeMethodInitializeActiveLabelsPipeline,
    update_pipeline: &FastIterativeMethodUpdatePipeline,
    pipeline_cache: &PipelineCache,
    pass: &mut ComputePass,
    bind_groups: &ReinitializeLevelSetBindGroupsItem,
    resolution: UVec3,
    workgroup_size: UVec3,
) {
    pass.push_debug_group("reinitialize_levelset");
    let init_pipeline = pipeline_cache
        .get_compute_pipeline(init_pipeline.pipeline)
        .unwrap();

    let init_labels_pipeline = pipeline_cache
        .get_compute_pipeline(init_labels_pipeline.pipeline)
        .unwrap();

    let update_pipeline = pipeline_cache
        .get_compute_pipeline(update_pipeline.pipeline)
        .unwrap();

    let workgroups = num_workgroups(resolution, workgroup_size);
    pass.set_pipeline(init_pipeline);
    pass.set_bind_group(0, &bind_groups.initialize_bind_group.bind_group, &[]);
    pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);

    pass.set_pipeline(init_labels_pipeline);
    pass.set_bind_group(
        0,
        &bind_groups.initialize_active_labels_bind_group.bind_group,
        &[],
    );
    pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);

    pass.push_debug_group("reinitialize_levelset_update");
    pass.set_pipeline(update_pipeline);
    pass.set_bind_group(0, &bind_groups.update_bind_group.bind_group, &[]);
    for _ in 0..5 {
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
    }
    pass.pop_debug_group();
    pass.pop_debug_group();
}
