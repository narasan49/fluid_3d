mod fluid;

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings},
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
};

use crate::fluid::{Fluid3d, Fluid3dPlugin};

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
        .add_plugins(Fluid3dPlugin)
        .add_systems(Startup, setup_dev_tools)
        .add_systems(Startup, setup_fluid)
        .run();
}

fn setup_dev_tools(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
