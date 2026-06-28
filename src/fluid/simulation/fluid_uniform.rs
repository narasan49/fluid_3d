use bevy::{
    material::descriptor::BindGroupLayoutDescriptor,
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayoutEntries, PipelineCache, ShaderStages,
            ShaderType, binding_types::uniform_buffer,
        },
        renderer::RenderDevice,
        uniform::{ComponentUniforms, DynamicUniformIndex, UniformComponentPlugin},
    },
    shader::load_shader_library,
};

use crate::fluid::Fluid3d;

pub struct FluidUniformPlugin;

impl Plugin for FluidUniformPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "fluid_uniform.wgsl");
        app.add_plugins((
            ExtractComponentPlugin::<FluidUniform>::default(),
            UniformComponentPlugin::<FluidUniform>::default(),
        ))
        .add_systems(Update, update_fluid_uniform);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        let bind_group_layout = BindGroupLayoutDescriptor::new(
            "fluid_uniform_bind_group_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                uniform_buffer::<FluidUniform>(true),
            ),
        );

        render_app
            .insert_resource(FluidUniformBindGroupLayout(bind_group_layout))
            .add_systems(
                Render,
                prepare_bind_group.in_set(RenderSystems::PrepareBindGroups),
            );
    }
}

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType)]
pub struct FluidUniform {
    pub dx: f32,
    pub dt: f32,
    pub rho: f32,
    pub gravity: Vec3,
    pub transform: Mat4,
    pub resolution: UVec3,
}

#[derive(Resource)]
pub struct FluidUniformBindGroupLayout(pub BindGroupLayoutDescriptor);

#[derive(Component)]
pub struct FluidUniformBindGroup {
    pub bind_group: BindGroup,
    pub index: u32,
}

fn prepare_bind_group(
    mut commands: Commands,
    fluid_uniform: Res<ComponentUniforms<FluidUniform>>,
    bind_group_layout: Res<FluidUniformBindGroupLayout>,
    query: Query<(Entity, &DynamicUniformIndex<FluidUniform>)>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
) {
    let fluid_uniform = fluid_uniform.uniforms();

    let bind_group = render_device.create_bind_group(
        "fluid_uniform_bind_grouo",
        &pipeline_cache.get_bind_group_layout(&bind_group_layout.0),
        &BindGroupEntries::single(fluid_uniform),
    );

    for (entity, uniform_index) in &query {
        commands.entity(entity).insert(FluidUniformBindGroup {
            bind_group: bind_group.clone(),
            index: uniform_index.index(),
        });
    }
}

fn update_fluid_uniform(
    mut query: Query<(&mut FluidUniform, &Fluid3d, &Transform)>,
    time: Res<Time>,
) {
    for (mut uniform, fluid, transform) in &mut query {
        uniform.dt = time.delta_secs();
        uniform.rho = fluid.rho;
        uniform.resolution = fluid.resolution;
        uniform.gravity = fluid.gravity;
        uniform.transform = transform.to_matrix();
    }
}
