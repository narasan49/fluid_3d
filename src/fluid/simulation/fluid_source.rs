use bevy::{
    prelude::*,
    render::extract_component::{ExtractComponent, ExtractComponentPlugin},
};

const MAX_FLUID_SOURCE: usize = 8;

pub struct FluidSourcePlugin;

impl Plugin for FluidSourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractComponentPlugin::<FluidSource>::default());
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

#[derive(Component)]
pub enum FluidSourceShape {
    Sphere { radius: u32 },
    Aabb { half_size: Vec3 },
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
