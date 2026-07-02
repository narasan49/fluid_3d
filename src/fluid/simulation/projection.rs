use bevy::{
    material::descriptor::{
        BindGroupLayoutDescriptor, CachedComputePipelineId, ComputePipelineDescriptor,
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_resource::{
            BindGroup, BindGroupLayoutEntries, ComputePass, ComputePipeline, PipelineCache,
            ShaderStages, StorageTextureAccess, TextureFormat,
            binding_types::{texture_storage_3d, uniform_buffer},
        },
    },
};

use crate::fluid::{
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
        ));
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub struct MultigridIterationGonfig {
    pub num_pre_smooth: u32,
    pub num_post_smooth: u32,
    pub num_coarsest: u32,
}

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
        pass.dispatch_workgroups(workgroups.x, workgroups.y, workgroups.z);
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
                    texture_storage_3d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    uniform_buffer(false),
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

        Self {
            gauss_seidel_red_pipeline,
            gauss_seidel_black_pipeline,
            residual_pipeline: (),
            restriction_pipeline: (),
            prolongation_pipeline: (),
            gauss_seidel_bind_group_layout,
            residual_bind_group_layout: (),
            restriction_bind_group_layout: (),
            prolongation_bind_group_layout: (),
        }
    }
}

#[derive(Component)]
pub struct MultigridProjectionBindGroups {
    gauss_seidel_bind_groups: Box<[BindGroup]>,
    residual_bind_groups: Box<[BindGroup]>,
    restriction_bind_groups: Box<[BindGroup]>,
    prolongation_bind_groups: Box<[BindGroup]>,
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

    commands
        .entity(entity)
        .insert(MultigridProjectionResources {
            x,
            b,
            levelset,
            fluid_fraction,
            residual,
        });
}
