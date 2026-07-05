use bevy::{
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_asset::RenderAssets,
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayoutEntries, ComputePass, PipelineCache,
            ShaderStages, StorageTextureAccess, TextureFormat, binding_types::texture_storage_3d,
        },
        renderer::RenderDevice,
        texture::GpuImage,
    },
};

use crate::fluid::{
    pipeline::is_pipeline_loaded, resources::FluidResources, workgroup::num_workgroups,
};

pub struct ExtrapolateVelocityPlugin;

impl Plugin for ExtrapolateVelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<ExtrapolateVelocityResource>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(
            Render,
            prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
        );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<ExtrapolateVelocityPipeline>();
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub struct ExtrapolateVelocityResource {
    pub u0: Handle<Image>,
    pub levelset_air0: Handle<Image>,
    pub velocity_fixed: [Handle<Image>; 2],
}

impl ExtrapolateVelocityResource {
    pub fn new(resources: &FluidResources) -> Self {
        Self {
            u0: resources.u0.clone(),
            levelset_air0: resources.levelset_air0.clone(),
            velocity_fixed: resources.velocity_fixed.clone(),
        }
    }
}

#[derive(Resource)]
pub struct ExtrapolateVelocityPipeline {
    initialize_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
    initialize_bind_group_layout: BindGroupLayoutDescriptor,
    update_bind_group_layout: BindGroupLayoutDescriptor,
}

impl ExtrapolateVelocityPipeline {
    pub fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.initialize_pipeline)
            && is_pipeline_loaded(pipeline_cache, self.update_pipeline)
    }

    pub fn dispatch(
        &self,
        pass: &mut ComputePass,
        pipeline_cache: &PipelineCache,
        bind_groups: &ExtrapolateVelocityBindGroups,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("extrapolate_velocity");
        let workgroups = num_workgroups(resolution, workgroup_size);
        let initialize_pipeline = pipeline_cache
            .get_compute_pipeline(self.initialize_pipeline)
            .unwrap();
        let update_pipeline = pipeline_cache
            .get_compute_pipeline(self.update_pipeline)
            .unwrap();

        pass.set_pipeline(initialize_pipeline);
        pass.set_bind_group(0, &bind_groups.initialize_bind_group, &[]);
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);

        pass.set_pipeline(update_pipeline);
        for _ in 0..3 {
            pass.set_bind_group(0, &bind_groups.update_bind_groups[0], &[]);
            pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);

            pass.set_bind_group(0, &bind_groups.update_bind_groups[1], &[]);
            pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        }

        pass.pop_debug_group();
    }
}

impl FromWorld for ExtrapolateVelocityPipeline {
    fn from_world(world: &mut World) -> Self {
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let initialize_bind_group_layout = BindGroupLayoutDescriptor::new(
            "extrapolate_velocity_initialize_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R8Uint, StorageTextureAccess::WriteOnly),
                ),
            ),
        );

        let update_bind_group_layout = BindGroupLayoutDescriptor::new(
            "extrapolate_velocity_update_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::ReadWrite),
                    texture_storage_3d(TextureFormat::R8Uint, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R8Uint, StorageTextureAccess::WriteOnly),
                ),
            ),
        );

        let initialize_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("extrapolate_velocity_initialize_pipeline".into()),
                layout: vec![initialize_bind_group_layout.clone()],
                shader: asset_server
                    .load("shaders/simulation/extrapolate_velocity/initialize.wgsl"),
                entry_point: Some("initialize".into()),
                ..default()
            });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("extrapolate_velocity_update_pipeline".into()),
            layout: vec![update_bind_group_layout.clone()],
            shader: asset_server.load("shaders/simulation/extrapolate_velocity/update.wgsl"),
            entry_point: Some("update".into()),
            ..default()
        });

        Self {
            initialize_pipeline,
            update_pipeline,
            initialize_bind_group_layout,
            update_bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct ExtrapolateVelocityBindGroups {
    initialize_bind_group: BindGroup,
    update_bind_groups: Vec<BindGroup>,
}

fn prepare_bind_groups(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    render_device: Res<RenderDevice>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    query: Query<(Entity, &ExtrapolateVelocityResource)>,
    pipeline: Res<ExtrapolateVelocityPipeline>,
) {
    for (entity, resource) in &query {
        let u0 = gpu_images.get(&resource.u0).unwrap();
        let levelset_air0 = gpu_images.get(&resource.levelset_air0).unwrap();
        let velocity_fixed0 = gpu_images.get(&resource.velocity_fixed[0]).unwrap();
        let velocity_fixed1 = gpu_images.get(&resource.velocity_fixed[1]).unwrap();

        let initialize_bind_group = render_device.create_bind_group(
            "extrapolate_velocity_initialize_bind_group_layout",
            &pipeline_cache.get_bind_group_layout(&pipeline.initialize_bind_group_layout),
            &BindGroupEntries::sequential((
                &levelset_air0.texture_view,
                &velocity_fixed0.texture_view,
            )),
        );

        let mut update_bind_groups = Vec::with_capacity(2);
        update_bind_groups.push(render_device.create_bind_group(
            "extrapolate_velocity_update_bind_group_layout_0",
            &pipeline_cache.get_bind_group_layout(&pipeline.update_bind_group_layout),
            &BindGroupEntries::sequential((
                &u0.texture_view,
                &velocity_fixed0.texture_view,
                &velocity_fixed1.texture_view,
            )),
        ));
        update_bind_groups.push(render_device.create_bind_group(
            "extrapolate_velocity_update_bind_group_layout_1",
            &pipeline_cache.get_bind_group_layout(&pipeline.update_bind_group_layout),
            &BindGroupEntries::sequential((
                &u0.texture_view,
                &velocity_fixed1.texture_view,
                &velocity_fixed0.texture_view,
            )),
        ));

        commands
            .entity(entity)
            .insert(ExtrapolateVelocityBindGroups {
                initialize_bind_group,
                update_bind_groups,
            });
    }
}
