pub mod build_vertex_buffer;
pub mod lookup_table;
pub mod setup;

use bevy::{
    prelude::*,
    render::{
        RenderApp,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
    },
};

use crate::marching_cubes::{
    build_vertex_buffer::BuildVertexBufferPlugin, setup::setup_marching_cubes_resources,
};

pub struct MarchingCubesPlugin;

impl Plugin for MarchingCubesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildVertexBufferPlugin)
            .add_plugins(ExtractComponentPlugin::<MarchingCubes>::default())
            .add_systems(Update, setup_marching_cubes_resources);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
    }
}

#[derive(Component, ExtractComponent, Clone)]
pub struct MarchingCubes {
    pub sdf: Handle<Image>,
    pub grad_sdf: Handle<Image>,
    pub resolution: UVec3,
}
