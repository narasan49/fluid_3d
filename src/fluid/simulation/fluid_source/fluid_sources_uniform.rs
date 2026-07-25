use bevy::{
    material::descriptor::BindGroupLayoutDescriptor,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_resource::{
            BindGroup, BindGroupEntries, BindGroupLayoutEntries, PipelineCache, ShaderStages,
            ShaderType, binding_types::uniform_buffer,
        },
        renderer::RenderDevice,
        uniform::{ComponentUniforms, DynamicUniformIndex},
    },
};

use crate::fluid::simulation::fluid_source::{
    FluidSource, FluidSourceShape, FluidSourceTiming, FluidSourceVelocity, MAX_FLUID_SOURCE,
};

#[derive(Clone, Copy, ShaderType, Default)]
pub struct FluidSourceData {
    mode: u32,
    shape: u32,
    position: Vec3,
    velosity: Vec3,
    shape_values: Vec3,
}

#[derive(Component, ExtractComponent, Clone, ShaderType, Default)]
pub struct FluidSourcesUniform {
    data: [FluidSourceData; MAX_FLUID_SOURCE],
    count: u32,
}

#[derive(Component, ExtractComponent, Clone, ShaderType, Default)]
pub struct FluidSourcesOnResetUniform {
    data: [FluidSourceData; MAX_FLUID_SOURCE],
    count: u32,
}

#[derive(Resource)]
pub struct FluidSourcesBindGroupLayout {
    pub bind_group_layout: BindGroupLayoutDescriptor,
}

impl FluidSourcesBindGroupLayout {
    pub(super) fn new() -> Self {
        let bind_group_layout = BindGroupLayoutDescriptor::new(
            "fluid_sources_uniform_bind_group_layout",
            &BindGroupLayoutEntries::single(
                ShaderStages::COMPUTE,
                uniform_buffer::<FluidSourcesUniform>(true),
            ),
        );

        Self { bind_group_layout }
    }
}

#[derive(Component)]
pub struct FluidSourcesUniformBindGroup {
    pub bind_group: BindGroup,
    pub index: u32,
}

#[derive(Component)]
pub struct FluidSourcesOnResetUniformBindGroup {
    pub bind_group: BindGroup,
    pub index: u32,
}

pub(super) fn prepare_bind_groups(
    mut commands: Commands,
    fluid_sources_uniform: Res<ComponentUniforms<FluidSourcesUniform>>,
    fluid_sources_on_reset_uniform: Res<ComponentUniforms<FluidSourcesOnResetUniform>>,
    bind_group_layout: Res<FluidSourcesBindGroupLayout>,
    query: Query<(
        Entity,
        &DynamicUniformIndex<FluidSourcesUniform>,
        &DynamicUniformIndex<FluidSourcesOnResetUniform>,
    )>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
) {
    let fluid_sources_uniform = fluid_sources_uniform.uniforms();
    let fluid_sources_on_reset_uniform = fluid_sources_on_reset_uniform.uniforms();
    let bind_group = render_device.create_bind_group(
        "fluid_sources_bind_group",
        &pipeline_cache.get_bind_group_layout(&bind_group_layout.bind_group_layout),
        &BindGroupEntries::single(fluid_sources_uniform),
    );
    let bind_group_on_reset = render_device.create_bind_group(
        "fluid_sources_on_reset_bind_group",
        &pipeline_cache.get_bind_group_layout(&bind_group_layout.bind_group_layout),
        &BindGroupEntries::single(fluid_sources_on_reset_uniform),
    );
    for (entity, index, index_on_reset) in &query {
        commands.entity(entity).insert((
            FluidSourcesUniformBindGroup {
                bind_group: bind_group.clone(),
                index: index.index(),
            },
            FluidSourcesOnResetUniformBindGroup {
                bind_group: bind_group_on_reset.clone(),
                index: index_on_reset.index(),
            },
        ));
    }
}

pub(super) fn update_fluid_sources_buffer(
    mut q_fluid: Query<(
        &mut FluidSourcesUniform,
        &mut FluidSourcesOnResetUniform,
        &Children,
    )>,
    q_sources: Query<(
        &FluidSource,
        &FluidSourceVelocity,
        &FluidSourceShape,
        &FluidSourceTiming,
        &Transform,
    )>,
) {
    for (mut fluid_sources_uniform, mut fluid_sources_on_reset_uniform, children) in &mut q_fluid {
        let mut count = 0;
        let mut count_on_reset = 0;
        let mut data = [FluidSourceData::default(); MAX_FLUID_SOURCE];
        let mut data_on_reset = [FluidSourceData::default(); MAX_FLUID_SOURCE];
        for &child in children {
            let Ok((source, source_velocity, source_shape, timing, transform)) =
                q_sources.get(child)
            else {
                continue;
            };
            if !source.active {
                continue;
            }

            match *timing {
                FluidSourceTiming::OnReset => {
                    if count_on_reset >= MAX_FLUID_SOURCE {
                        warn!(
                            "maximum fluid sources per fluid entity ({:?}) exceeded",
                            count_on_reset
                        );
                        break;
                    }

                    data_on_reset[count_on_reset] = FluidSourceData {
                        mode: source.mode.to_u32(),
                        shape: source_shape.to_u32(),
                        position: transform.translation,
                        velosity: source_velocity.0,
                        shape_values: source_shape.data(),
                    };
                    count_on_reset += 1;
                }
                FluidSourceTiming::NotOnReset => {
                    if count >= MAX_FLUID_SOURCE {
                        warn!(
                            "maximum fluid sources per fluid entity ({:?}) exceeded",
                            count
                        );
                        break;
                    }

                    data[count] = FluidSourceData {
                        mode: source.mode.to_u32(),
                        shape: source_shape.to_u32(),
                        position: transform.translation,
                        velosity: source_velocity.0,
                        shape_values: source_shape.data(),
                    };
                    count += 1;
                }
            }
        }
        fluid_sources_uniform.data = data;
        fluid_sources_uniform.count = count as u32;
        fluid_sources_on_reset_uniform.data = data_on_reset;
        fluid_sources_on_reset_uniform.count = count_on_reset as u32;
    }
}
