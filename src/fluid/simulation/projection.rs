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
            BindGroup, BindGroupEntries, BindGroupLayoutEntries, ComputePass, ComputePipeline,
            PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat, UniformBuffer,
            binding_types::{texture_storage_3d, uniform_buffer},
        },
        renderer::{RenderDevice, RenderQueue},
        texture::GpuImage,
    },
};

use crate::fluid::{
    pipeline::is_pipeline_loaded,
    resources::{FluidResources, new_texture_storage_3d},
    simulation::fluid_uniform::{FluidUniformBindGroup, FluidUniformBindGroupLayout},
    workgroup::num_workgroups,
};

pub struct MultigridProjectionPassPlugin;

impl Plugin for MultigridProjectionPassPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<MultigridProjectionResources>::default(),
            ExtractComponentPlugin::<MultigridIterationGonfig>::default(),
            ExtractComponentPlugin::<MultigridNumLevels>::default(),
        ));

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

        render_app.init_resource::<MultigridProjectionPipeline>();
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub struct MultigridIterationGonfig {
    pub num_pre_smooth: u32,
    pub num_post_smooth: u32,
    pub num_coarsest: u32,
}

#[derive(Component, ExtractComponent, Clone)]
pub struct MultigridNumLevels(pub usize);

#[derive(Component, ExtractComponent, Clone)]
pub struct MultigridProjectionResources {
    x: Vec<Handle<Image>>,
    b: Vec<Handle<Image>>,
    levelset: Vec<Handle<Image>>,
    fluid_fraction: Vec<Handle<Image>>,
    residual: Vec<Handle<Image>>,
}

#[derive(Resource)]
pub struct MultigridProjectionPipeline {
    gauss_seidel_red_pipeline: CachedComputePipelineId,
    gauss_seidel_black_pipeline: CachedComputePipelineId,
    residual_pipeline: CachedComputePipelineId,
    restriction_pipeline: CachedComputePipelineId,
    prolongation_pipeline: CachedComputePipelineId,
    gauss_seidel_bind_group_layout: BindGroupLayoutDescriptor,
    residual_bind_group_layout: BindGroupLayoutDescriptor,
    restriction_bind_group_layout: BindGroupLayoutDescriptor,
    prolongation_bind_group_layout: BindGroupLayoutDescriptor,
}

impl MultigridProjectionPipeline {
    pub fn is_ready(&self, pipeline_cache: &PipelineCache) -> bool {
        is_pipeline_loaded(pipeline_cache, self.gauss_seidel_red_pipeline)
            && is_pipeline_loaded(pipeline_cache, self.gauss_seidel_black_pipeline)
            && is_pipeline_loaded(pipeline_cache, self.residual_pipeline)
            && is_pipeline_loaded(pipeline_cache, self.restriction_pipeline)
            && is_pipeline_loaded(pipeline_cache, self.prolongation_pipeline)
    }

    pub fn dispatch(
        &self,
        pipeline_cache: &PipelineCache,
        pass: &mut ComputePass,
        bind_groups: &MultigridProjectionBindGroups,
        uniform_bind_group: &FluidUniformBindGroup,
        config: &MultigridIterationGonfig,
        num_levels: usize,
        resolution: UVec3,
        workgroup_size: UVec3,
    ) {
        pass.push_debug_group("multigrid_projection");
        let gauss_seidel_red_pipeline = pipeline_cache
            .get_compute_pipeline(self.gauss_seidel_red_pipeline)
            .unwrap();
        let gauss_seidel_black_pipeline = pipeline_cache
            .get_compute_pipeline(self.gauss_seidel_black_pipeline)
            .unwrap();
        let residual_pipeline = pipeline_cache
            .get_compute_pipeline(self.residual_pipeline)
            .unwrap();
        let restriction_pipeline = pipeline_cache
            .get_compute_pipeline(self.restriction_pipeline)
            .unwrap();
        let prolongation_pipeline = pipeline_cache
            .get_compute_pipeline(self.prolongation_pipeline)
            .unwrap();

        pass.set_bind_group(
            1,
            &uniform_bind_group.bind_group,
            &[uniform_bind_group.index],
        );

        self.v_cycle(
            pass,
            gauss_seidel_red_pipeline,
            gauss_seidel_black_pipeline,
            residual_pipeline,
            restriction_pipeline,
            prolongation_pipeline,
            bind_groups,
            config,
            resolution,
            workgroup_size,
            num_levels,
            0,
        );

        pass.pop_debug_group();
    }

    fn v_cycle(
        &self,
        pass: &mut ComputePass,
        gauss_seidel_red_pipeline: &ComputePipeline,
        gauss_seidel_black_pipeline: &ComputePipeline,
        residual_pipeline: &ComputePipeline,
        restriction_pipeline: &ComputePipeline,
        prolongation_pipeline: &ComputePipeline,
        bind_groups: &MultigridProjectionBindGroups,
        config: &MultigridIterationGonfig,
        resolution: UVec3,
        workgroup_size: UVec3,
        num_levels: usize,
        level: usize,
    ) {
        let workgroups = num_workgroups(resolution, workgroup_size);
        let workgroups_xyz = num_workgroups(resolution + UVec3::ONE, workgroup_size);
        if level == num_levels - 1 {
            pass.push_debug_group("solve_coarsest");
            pass.set_bind_group(0, &bind_groups.gauss_seidel_bind_groups[level], &[]);
            for _ in 0..config.num_coarsest {
                pass.set_pipeline(gauss_seidel_red_pipeline);
                pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
                pass.set_pipeline(gauss_seidel_black_pipeline);
                pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
            }
            pass.pop_debug_group();
            return;
        }
        pass.push_debug_group("pre_smooth");
        pass.set_bind_group(0, &bind_groups.gauss_seidel_bind_groups[level], &[]);
        for _ in 0..config.num_pre_smooth {
            pass.set_pipeline(gauss_seidel_red_pipeline);
            pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
            pass.set_pipeline(gauss_seidel_black_pipeline);
            pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        }
        pass.pop_debug_group();

        pass.push_debug_group("residual");
        pass.set_pipeline(residual_pipeline);
        pass.set_bind_group(0, &bind_groups.residual_bind_groups[level], &[]);
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        pass.pop_debug_group();

        pass.push_debug_group("restriction");
        pass.set_pipeline(restriction_pipeline);
        pass.set_bind_group(0, &bind_groups.restriction_bind_groups[level], &[]);
        pass.dispatch_workgroups(workgroups_xyz.x, workgroups_xyz.y, workgroups_xyz.z);
        pass.pop_debug_group();

        self.v_cycle(
            pass,
            gauss_seidel_red_pipeline,
            gauss_seidel_black_pipeline,
            residual_pipeline,
            restriction_pipeline,
            prolongation_pipeline,
            bind_groups,
            config,
            resolution / UVec3::splat(2),
            workgroup_size,
            num_levels,
            level + 1,
        );

        pass.push_debug_group("prolongation");
        pass.set_pipeline(prolongation_pipeline);
        pass.set_bind_group(0, &bind_groups.prolongation_bind_groups[level], &[]);
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        pass.pop_debug_group();

        pass.push_debug_group("post_smooth");
        pass.set_bind_group(0, &bind_groups.gauss_seidel_bind_groups[level], &[]);
        for _ in 0..config.num_post_smooth {
            pass.set_pipeline(gauss_seidel_red_pipeline);
            pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
            pass.set_pipeline(gauss_seidel_black_pipeline);
            pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
        }
        pass.pop_debug_group();
    }
}

impl FromWorld for MultigridProjectionPipeline {
    fn from_world(world: &mut World) -> Self {
        let pipeline_cache = world.resource::<PipelineCache>();
        let asset_server = world.resource::<AssetServer>();

        let uniform_bind_group_layout = world.resource::<FluidUniformBindGroupLayout>();
        let gauss_seidel_bind_group_layout = BindGroupLayoutDescriptor::new(
            "gauss_seidel_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadWrite),
                    uniform_buffer::<f32>(false),
                ),
            ),
        );
        let residual_bind_group_layout = BindGroupLayoutDescriptor::new(
            "residual_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::WriteOnly),
                    uniform_buffer::<f32>(false),
                ),
            ),
        );
        let restriction_bind_group_layout = BindGroupLayoutDescriptor::new(
            "restriction_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::WriteOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::WriteOnly),
                    texture_storage_3d(TextureFormat::Rgba16Float, StorageTextureAccess::WriteOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::WriteOnly),
                ),
            ),
        );
        let prolongation_bind_group_layout = BindGroupLayoutDescriptor::new(
            "prolongation_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadWrite),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                ),
            ),
        );

        let gauss_seidel_red_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("gauss_seidel_red_pipeline".into()),
                layout: vec![
                    gauss_seidel_bind_group_layout.clone(),
                    uniform_bind_group_layout.0.clone(),
                ],
                shader: asset_server.load("shaders/simulation/projection/gauss_seidel.wgsl"),
                entry_point: Some("gauss_seidel_red".into()),
                ..default()
            });
        let gauss_seidel_black_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("gauss_seidel_black_pipeline".into()),
                layout: vec![
                    gauss_seidel_bind_group_layout.clone(),
                    uniform_bind_group_layout.0.clone(),
                ],
                shader: asset_server.load("shaders/simulation/projection/gauss_seidel.wgsl"),
                entry_point: Some("gauss_seidel_black".into()),
                ..default()
            });
        let residual_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("residual_pipeline".into()),
            layout: vec![
                residual_bind_group_layout.clone(),
                uniform_bind_group_layout.0.clone(),
            ],
            shader: asset_server.load("shaders/simulation/projection/residual.wgsl"),
            entry_point: Some("residual".into()),
            ..default()
        });
        let restriction_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("restriction_pipeline".into()),
                layout: vec![
                    restriction_bind_group_layout.clone(),
                    uniform_bind_group_layout.0.clone(),
                ],
                shader: asset_server.load("shaders/simulation/projection/restriction.wgsl"),
                entry_point: Some("restriction".into()),
                ..default()
            });
        let prolongation_pipeline =
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("prolongation_pipeline".into()),
                layout: vec![
                    prolongation_bind_group_layout.clone(),
                    uniform_bind_group_layout.0.clone(),
                ],
                shader: asset_server.load("shaders/simulation/projection/prolongation.wgsl"),
                entry_point: Some("prolongation".into()),
                ..default()
            });

        Self {
            gauss_seidel_red_pipeline,
            gauss_seidel_black_pipeline,
            residual_pipeline,
            restriction_pipeline,
            prolongation_pipeline,
            gauss_seidel_bind_group_layout,
            residual_bind_group_layout,
            restriction_bind_group_layout,
            prolongation_bind_group_layout,
        }
    }
}

#[derive(Component)]
pub struct MultigridProjectionBindGroups {
    gauss_seidel_bind_groups: Vec<BindGroup>,
    residual_bind_groups: Vec<BindGroup>,
    restriction_bind_groups: Vec<BindGroup>,
    prolongation_bind_groups: Vec<BindGroup>,
}

pub fn setup_multigrid_resources(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    entity: Entity,
    resources: &FluidResources,
    resolution: UVec3,
) {
    let num_levels = ((resolution.min_element() as f32).log2() as usize)
        .saturating_sub(1)
        .max(1);

    let mut x = Vec::<Handle<Image>>::with_capacity(num_levels);
    let mut b = Vec::<Handle<Image>>::with_capacity(num_levels);
    let mut levelset = Vec::<Handle<Image>>::with_capacity(num_levels);
    let mut fluid_fraction = Vec::<Handle<Image>>::with_capacity(num_levels);
    let mut residual = Vec::<Handle<Image>>::with_capacity(num_levels);

    x.push(resources.p.clone());
    b.push(resources.div.clone());
    levelset.push(resources.levelset_air0.clone());
    fluid_fraction.push(resources.fluid_fraction.clone());
    residual.push(new_texture_storage_3d(
        images,
        resolution,
        TextureFormat::R32Float,
    ));

    let mut resolution = resolution;
    for _ in 1..num_levels {
        resolution /= 2;
        x.push(new_texture_storage_3d(
            images,
            resolution,
            TextureFormat::R32Float,
        ));

        b.push(new_texture_storage_3d(
            images,
            resolution,
            TextureFormat::R32Float,
        ));

        levelset.push(new_texture_storage_3d(
            images,
            resolution,
            TextureFormat::R32Float,
        ));

        fluid_fraction.push(new_texture_storage_3d(
            images,
            resolution,
            TextureFormat::Rgba16Float,
        ));

        residual.push(new_texture_storage_3d(
            images,
            resolution,
            TextureFormat::R32Float,
        ));
    }

    commands.entity(entity).insert((
        MultigridProjectionResources {
            x,
            b,
            levelset,
            fluid_fraction,
            residual,
        },
        MultigridNumLevels(num_levels),
        MultigridIterationGonfig {
            num_pre_smooth: 2,
            num_post_smooth: 2,
            num_coarsest: 3,
        },
    ));
}

fn prepare_bind_groups(
    mut commands: Commands,
    pipeline: Res<MultigridProjectionPipeline>,
    query: Query<(Entity, &MultigridProjectionResources, &MultigridNumLevels)>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    pipeline_cache: Res<PipelineCache>,
    gpu_images: Res<RenderAssets<GpuImage>>,
) {
    for (entity, resources, levels) in &query {
        let mut gauss_seidel_bind_groups = Vec::with_capacity(levels.0);
        let mut residual_bind_groups = Vec::with_capacity(levels.0);
        let mut restriction_bind_groups = Vec::with_capacity(levels.0);
        let mut prolongation_bind_groups = Vec::with_capacity(levels.0);
        for i in 0..levels.0 {
            let mut dx_scale_buffer = UniformBuffer::from((1 << i) as f32);
            dx_scale_buffer.write_buffer(&render_device, &render_queue);
            let dx_scale = dx_scale_buffer.binding().unwrap();
            let b = gpu_images.get(&resources.b[i]).unwrap();
            let fluid_fraction = gpu_images.get(&resources.fluid_fraction[i]).unwrap();
            let levelset = gpu_images.get(&resources.levelset[i]).unwrap();
            let residual = gpu_images.get(&resources.residual[i]).unwrap();
            let x = gpu_images.get(&resources.x[i]).unwrap();
            gauss_seidel_bind_groups.push(render_device.create_bind_group(
                format!("gauss_seidel_bind_group_{i}").as_str(),
                &pipeline_cache.get_bind_group_layout(&pipeline.gauss_seidel_bind_group_layout),
                &BindGroupEntries::sequential((
                    &b.texture_view,
                    &levelset.texture_view,
                    &fluid_fraction.texture_view,
                    &x.texture_view,
                    dx_scale.clone(),
                )),
            ));

            if i == levels.0 - 1 {
                continue;
            }

            residual_bind_groups.push(render_device.create_bind_group(
                format!("residual_bind_group_{i}").as_str(),
                &pipeline_cache.get_bind_group_layout(&pipeline.residual_bind_group_layout),
                &BindGroupEntries::sequential((
                    &b.texture_view,
                    &levelset.texture_view,
                    &fluid_fraction.texture_view,
                    &x.texture_view,
                    &residual.texture_view,
                    dx_scale,
                )),
            ));

            let b_plus = gpu_images.get(&resources.b[i + 1]).unwrap();
            let levelset_plus = gpu_images.get(&resources.levelset[i + 1]).unwrap();
            let fluid_fraction_plus = gpu_images.get(&resources.fluid_fraction[i + 1]).unwrap();
            let x_plus = gpu_images.get(&resources.x[i + 1]).unwrap();

            restriction_bind_groups.push(render_device.create_bind_group(
                format!("restriction_bind_group_{i}").as_str(),
                &pipeline_cache.get_bind_group_layout(&pipeline.restriction_bind_group_layout),
                &BindGroupEntries::sequential((
                    &residual.texture_view,
                    &levelset.texture_view,
                    &fluid_fraction.texture_view,
                    &b_plus.texture_view,
                    &levelset_plus.texture_view,
                    &fluid_fraction_plus.texture_view,
                    &x_plus.texture_view,
                )),
            ));

            prolongation_bind_groups.push(render_device.create_bind_group(
                format!("prolongation_bind_group_{i}").as_str(),
                &pipeline_cache.get_bind_group_layout(&pipeline.prolongation_bind_group_layout),
                &BindGroupEntries::sequential((
                    &x.texture_view,
                    &x_plus.texture_view,
                    &levelset.texture_view,
                )),
            ));
        }

        commands
            .entity(entity)
            .insert(MultigridProjectionBindGroups {
                gauss_seidel_bind_groups,
                residual_bind_groups,
                restriction_bind_groups,
                prolongation_bind_groups,
            });
    }
}
