mod fluid;
mod marching_cubes;

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings},
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};

use crate::{
    fluid::{Fluid3d, Fluid3dPlugin, resources::FluidResources},
    marching_cubes::{MarchingCubes, MarchingCubesPlugin},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(Box::new(WgpuSettings {
                    backends: Some(Backends::DX12),
                    ..default()
                })),
                ..default()
            }),
            FreeCameraPlugin,
            InfiniteGridPlugin,
        ))
        .add_plugins((Fluid3dPlugin, MarchingCubesPlugin))
        .add_systems(Startup, setup_dev_tools)
        .add_systems(Startup, setup_fluid)
        .add_systems(Update, setup_fluid_render)
        .run();
}

fn setup_dev_tools(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));

    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));
}

fn setup_fluid(mut commands: Commands) {
    commands.spawn(Fluid3d {
        resolution: UVec3::new(64, 64, 64),
        rho: 997.0,
        gravity: -Vec3::Y * 9.8,
    });
}

// レベルセットテクスチャをMarchingCubesに渡して描画する
fn setup_fluid_render(
    mut commands: Commands,
    query: Query<(Entity, &FluidResources, &Fluid3d), Added<FluidResources>>,
) {
    for (entity, resources, fluid) in &query {
        commands.entity(entity).insert(MarchingCubes {
            sdf: resources.levelset_air0.clone(),
            grad_sdf: resources.grad_levelset_air.clone(),
            resolution: fluid.resolution,
        });
    }
}
