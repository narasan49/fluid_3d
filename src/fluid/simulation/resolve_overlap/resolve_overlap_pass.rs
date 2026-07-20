use bevy::{
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        render_asset::RenderAssets,
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayoutEntries, ComputePass, PipelineCache,
            ShaderStages, ShaderType, StorageTextureAccess, TextureFormat, UniformBuffer,
            binding_types::{texture_storage_3d, uniform_buffer},
        },
        renderer::{RenderDevice, RenderQueue},
        texture::GpuImage,
    },
};

use crate::fluid::{
    pipeline::{FluidPipeline, is_pipeline_loaded},
    resources::FluidResources,
    simulation::{
        fluid_uniform::{FluidUniform, FluidUniformBindGroup, FluidUniformBindGroupLayout},
        resolve_overlap::OverlappedFluids,
    },
    workgroup::num_workgroups,
};

pub struct ResolveOverlapPassPlugin;

impl Plugin for ResolveOverlapPassPlugin {
    fn build(&self, app: &mut App) {
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
        render_app.init_resource::<ResolveOverlapPipeline>();
    }
}

#[derive(Clone, Copy, ShaderType)]
pub struct OtherFluidUniform {
    pub inverse_transform: Mat4,
    pub half_size: Vec3,
}

#[derive(Resource)]
pub struct ResolveOverlapPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayoutDescriptor,
}

impl ResolveOverlapPipeline {
    pub fn dispatch(
        &self,
        pass: &mut ComputePass,
        pipeline_cache: &PipelineCache,
        bind_groups: &ResolveOverlapBindGroups,
        uniform_bind_group: &FluidUniformBindGroup,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        let workgroups = num_workgroups(resolution, workgroup_size);
        pass.push_debug_group("resolve_overlap");
        let pipeline = pipeline_cache.get_compute_pipeline(self.pipeline).unwrap();

        pass.set_pipeline(pipeline);
        pass.set_bind_group(
            1,
            &uniform_bind_group.bind_group,
            &[uniform_bind_group.index],
        );
        for other_bind_group in &bind_groups.other_bind_groups {
            pass.set_bind_group(0, other_bind_group, &[]);
            pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        }
        pass.pop_debug_group();
    }
}

impl FluidPipeline for ResolveOverlapPipeline {
    fn bind_group_layoput(&self) -> &BindGroupLayoutDescriptor {
        &self.bind_group_layout
    }

    fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.pipeline)
    }
}

impl FromWorld for ResolveOverlapPipeline {
    fn from_world(world: &mut World) -> Self {
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let bind_group_layout = BindGroupLayoutDescriptor::new(
            "resolve_overlap_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadWrite),
                    texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::ReadWrite),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadWrite),
                    texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::ReadOnly),
                    uniform_buffer::<OtherFluidUniform>(false),
                ),
            ),
        );
        let uniform_bind_group_layout = world.resource::<FluidUniformBindGroupLayout>();

        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("resolve_overlap_pipeline".into()),
            layout: vec![
                bind_group_layout.clone(),
                uniform_bind_group_layout.0.clone(),
            ],
            shader: asset_server.load("shaders/simulation/resolve_overlap.wgsl"),
            entry_point: Some("resolve_overlap".into()),
            ..default()
        });

        Self {
            pipeline,
            bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct ResolveOverlapBindGroups {
    other_bind_groups: Vec<BindGroup>,
}

fn prepare_bind_groups(
    mut commands: Commands,
    query: Query<(Entity, &FluidResources, &FluidUniform, &OverlappedFluids)>,
    pipeline: Res<ResolveOverlapPipeline>,
    pipeline_cache: Res<PipelineCache>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    gpu_images: Res<RenderAssets<GpuImage>>,
) {
    for (entity, resources, _, others) in &query {
        let levelset_air_this = gpu_images.get(&resources.levelset_air0).unwrap();
        let u0_this = gpu_images.get(&resources.u0).unwrap();
        let mut other_bind_groups = Vec::with_capacity(others.0.len());
        for other_entity in &others.0 {
            let Ok((_, other_resoures, other_uniform, _)) = query.get(*other_entity) else {
                continue;
            };
            let levelset_air_other = gpu_images.get(&other_resoures.levelset_air0).unwrap();
            let u0_other = gpu_images.get(&other_resoures.u0).unwrap();
            let other_fluid_uniform = OtherFluidUniform {
                inverse_transform: other_uniform.transform.inverse(),
                half_size: other_uniform.half_size,
            };
            let mut other_fluid_uniform = UniformBuffer::from(other_fluid_uniform);
            other_fluid_uniform.write_buffer(&render_device, &render_queue);
            let other_fluid_uniform = other_fluid_uniform.binding().unwrap();
            other_bind_groups.push(render_device.create_bind_group(
                "resolve_overlap_bind_group",
                &pipeline_cache.get_bind_group_layout(&pipeline.bind_group_layout),
                &BindGroupEntries::sequential((
                    &levelset_air_this.texture_view,
                    &u0_this.texture_view,
                    &levelset_air_other.texture_view,
                    &u0_other.texture_view,
                    other_fluid_uniform,
                )),
            ));
        }

        commands
            .entity(entity)
            .insert(ResolveOverlapBindGroups { other_bind_groups });
    }
}
