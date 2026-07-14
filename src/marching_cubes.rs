pub mod build_vertex_buffer;
pub mod draw;
pub mod lookup_table;
pub mod setup;

use bevy::{
    prelude::*,
    render::extract_component::{ExtractComponent, ExtractComponentPlugin},
};

use crate::marching_cubes::{
    build_vertex_buffer::BuildVertexBufferPlugin, draw::MarchingCubesDrawPlugin,
    setup::setup_marching_cubes_resources,
};

pub struct MarchingCubesPlugin;

impl Plugin for MarchingCubesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BuildVertexBufferPlugin, MarchingCubesDrawPlugin))
            .add_plugins(ExtractComponentPlugin::<MarchingCubes>::default())
            .add_systems(Update, setup_marching_cubes_resources);
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub struct MarchingCubes {
    pub grad_sdf: Handle<Image>,
    pub resolution: UVec3,
    pub half_size: Vec3,
}
