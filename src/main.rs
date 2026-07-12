mod fluid;
mod game;
mod marching_cubes;

use avian3d::{
    PhysicsPlugins,
    collision::collider::IntoCollider,
    dynamics::{integrator::Gravity, rigid_body::RigidBody},
};
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
    game::character_controller::{CharacterController, CharacterControllerPlugin},
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
        .add_plugins(PhysicsPlugins::default().with_length_unit(10.0))
        .add_plugins((
            Fluid3dPlugin,
            MarchingCubesPlugin,
            CharacterControllerPlugin,
        ))
        .add_systems(Startup, setup_dev_tools)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, setup_fluid_render)
        .insert_resource(Gravity(9.8 * Vec3::NEG_Y))
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

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Fluid3d {
            resolution: UVec3::new(64, 64, 64),
            rho: 997.0,
            gravity: -Vec3::Y * 9.8,
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));

    let ground_shape = Cuboid::new(2.0, 0.1, 2.0);

    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(ground_shape)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        ground_shape.collider(),
        Transform::from_translation(Vec3::new(0.0, -0.4, 0.0)),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));

    let capsule = Capsule3d::new(0.05, 0.1);
    commands.spawn((
        Name::new("PlayerCapsule"),
        Transform::from_xyz(0.0, -0.25, 0.5),
        Mesh3d(meshes.add(capsule)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.0))),
        capsule.collider(),
        CharacterController,
        RigidBody::Kinematic,
    ));

    let cube = Cuboid::from_size(Vec3::splat(0.2));
    commands.spawn((
        Name::new("Cube"),
        Transform::from_xyz(0.2, -0.2, -0.2),
        Mesh3d(meshes.add(cube)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        cube.collider(),
        RigidBody::Kinematic,
    ));
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
