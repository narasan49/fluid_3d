mod fluid;
mod game;
mod marching_cubes;
pub mod rigid_body;

use avian3d::{
    PhysicsPlugins,
    collision::collider::IntoCollider,
    dynamics::{
        integrator::Gravity,
        rigid_body::{LockedAxes, RigidBody},
    },
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
    fluid::{
        Fluid3d, Fluid3dPlugin, GridLength,
        resources::FluidResources,
        simulation::{
            fluid_source::{FluidSource, FluidSourceMode, FluidSourceShape, FluidSourceVelocity},
            solid_to_fluid::SolidShapeOnFluid,
        },
    },
    game::{
        character_controller::{CharacterController, CharacterControllerPlugin},
        solid_body_motion::{MovingObject, update_moving_object},
    },
    marching_cubes::{MarchingCubes, MarchingCubesPlugin},
    rigid_body::custom_collider::TriangularPrism,
};

const LENGTH_UNIT: f32 = 64.0;

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
        .add_plugins(PhysicsPlugins::default().with_length_unit(LENGTH_UNIT))
        .add_plugins((
            Fluid3dPlugin,
            MarchingCubesPlugin,
            CharacterControllerPlugin,
        ))
        .add_systems(Startup, setup_dev_tools)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, setup_fluid_render)
        .add_systems(Update, update_moving_object)
        .insert_resource(Gravity(9.8 * Vec3::NEG_Y))
        .insert_resource(GridLength(1.0 / LENGTH_UNIT))
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
    grid_length: Res<GridLength>,
) {
    // 流体。底面をy=0に設定する。
    let resolution = UVec3::new(128, 32, 64);
    let fluid_half_size = 0.5 * resolution.as_vec3() * (grid_length.0 as f32);
    commands
        .spawn((
            Fluid3d {
                resolution,
                rho: 997.0,
                gravity: 9.8 * Vec3::NEG_Y,
            },
            Transform::from_translation(Vec3::new(0.0, fluid_half_size.y, 0.0)),
        ))
        .with_children(|commands| {
            commands.spawn((
                FluidSource {
                    avtive: true,
                    mode: FluidSourceMode::Source,
                },
                FluidSourceShape::Aabb {
                    half_size: Vec3::splat(0.05),
                },
                FluidSourceVelocity(Vec3::NEG_Y * 3.0),
                Transform::from_translation(Vec3::new(-0.4, 0.1, 0.0)),
            ));
        });

    let material_terrain = materials.add(Color::srgb(0.8, 0.8, 0.8));
    // 上面をy=0にする
    let ground_shape = Cuboid::new(2.0, 0.1, 2.0);
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(ground_shape)),
        MeshMaterial3d(material_terrain.clone()),
        ground_shape.collider(),
        SolidShapeOnFluid::Cuboid(ground_shape),
        Transform::default().with_translation(Vec3::new(0.0, -ground_shape.half_size.y, 0.0)),
        RigidBody::Static,
    ));

    let slope = Extrusion::<Triangle2d>::new(
        Triangle2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 0.4),
        ),
        1.0,
    );
    commands.spawn((
        Name::new("Slope"),
        Mesh3d(meshes.add(slope)),
        SolidShapeOnFluid::TriangularPrism(slope),
        TriangularPrism::from(slope).collider(),
        MeshMaterial3d(material_terrain.clone()),
        RigidBody::Kinematic,
    ));

    let second_floor = Cuboid::new(2.0, 0.1, 0.5);
    commands.spawn((
        Name::new("2ndFloor"),
        Mesh3d(meshes.add(second_floor)),
        MeshMaterial3d(material_terrain.clone()),
        second_floor.collider(),
        Transform::default().with_translation(Vec3::new(
            0.0,
            -second_floor.half_size.y + 0.4,
            -1.0,
        )),
        RigidBody::Static,
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));

    let capsule = Capsule3d::new(0.05, 0.1);
    commands.spawn((
        Name::new("PlayerCapsule"),
        Transform::from_xyz(0.0, capsule.half_length + capsule.radius + 10.0, 0.5),
        Mesh3d(meshes.add(capsule)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.0))),
        capsule.collider(),
        SolidShapeOnFluid::Capsule(capsule),
        CharacterController,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
    ));

    let cube = Cuboid::from_size(Vec3::new(0.2, 0.2, 0.5));
    commands.spawn((
        Name::new("Cube"),
        Transform::default().with_translation(Vec3::new(-0.8, cube.half_size.y, -0.2)),
        Mesh3d(meshes.add(cube)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        cube.collider(),
        SolidShapeOnFluid::Cuboid(cube),
        RigidBody::Kinematic,
        MovingObject,
    ));
}

// レベルセットテクスチャをMarchingCubesに渡して描画する
fn setup_fluid_render(
    mut commands: Commands,
    query: Query<(Entity, &FluidResources, &Fluid3d), Added<FluidResources>>,
    grid_length: Res<GridLength>,
) {
    for (entity, resources, fluid) in &query {
        let half_size = 0.5 * fluid.resolution.as_vec3() * (grid_length.0 as f32);
        commands.entity(entity).insert(MarchingCubes {
            grad_sdf: resources.levelset_and_grad_air.clone(),
            resolution: fluid.resolution,
            half_size,
        });
    }
}
