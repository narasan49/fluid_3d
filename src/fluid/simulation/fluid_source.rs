pub mod fluid_sources_uniform;
pub mod update_fluid_sources;

use bevy::{
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
        uniform::UniformComponentPlugin,
    },
};

use fluid_sources_uniform::FluidSourcesBindGroupLayout;

use crate::fluid::simulation::fluid_source::fluid_sources_uniform::FluidSourcesUniform;

const MAX_FLUID_SOURCE: usize = 8;

pub struct FluidSourcePlugin;

impl Plugin for FluidSourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<FluidSource>::default(),
            ExtractComponentPlugin::<FluidSourcesUniform>::default(),
            UniformComponentPlugin::<FluidSourcesUniform>::default(),
        ))
        .add_systems(Update, fluid_sources_uniform::update_fluid_sources_buffer);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        let fluid_sources_bind_group_layout = FluidSourcesBindGroupLayout::new();
        render_app
            .add_systems(
                Render,
                fluid_sources_uniform::prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            )
            .insert_resource(fluid_sources_bind_group_layout);
    }
}

#[derive(Component, ExtractComponent, Clone)]
#[require(Transform, FluidSourceShape, FluidSourceVelocity)]
pub struct FluidSource {
    pub avtive: bool,
    pub mode: FluidSourceMode,
}

#[derive(Component, Default, Clone)]
pub enum FluidSourceMode {
    #[default]
    Source,
    Sink,
}

impl FluidSourceMode {
    pub fn to_u32(&self) -> u32 {
        match self {
            FluidSourceMode::Source => 0,
            FluidSourceMode::Sink => 1,
        }
    }
}

#[derive(Component)]
pub enum FluidSourceShape {
    Sphere { radius: f32 },
    Aabb { half_size: Vec3 },
}

impl FluidSourceShape {
    pub fn to_u32(&self) -> u32 {
        match self {
            FluidSourceShape::Sphere { radius: _ } => 0,
            FluidSourceShape::Aabb { half_size: _ } => 1,
        }
    }

    pub fn data(&self) -> Vec3 {
        match self {
            FluidSourceShape::Sphere { radius } => Vec3::new(*radius, 0.0, 0.0),
            FluidSourceShape::Aabb { half_size } => *half_size,
        }
    }
}

impl Default for FluidSourceShape {
    fn default() -> Self {
        Self::Aabb {
            half_size: Vec3::splat(0.5),
        }
    }
}

#[derive(Component, Default)]
pub struct FluidSourceVelocity(pub Vec3);
