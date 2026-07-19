pub mod diagnostics;
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
    diagnostics::fluid_bounds::{FluidBounds, FluidBoundsPlugin},
    fluid::{
        BoundaryConditions, Fluid3d, Fluid3dPlugin, FluidBoundaryMethod, GridLength,
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
            FluidBoundsPlugin,
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
    commands.spawn((
        Fluid3d {
            resolution,
            rho: 997.0,
            gravity: 9.8 * Vec3::NEG_Y,
        },
        FluidBounds,
        BoundaryConditions {
            y_max: FluidBoundaryMethod::Open,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, fluid_half_size.y, 0.0)),
    ));

    let resolution = UVec3::new(32, 32, 64);
    let source_fluid_half_size = 0.5 * resolution.as_vec3() * (grid_length.0 as f32);
    commands
        .spawn((
            Fluid3d {
                resolution,
                rho: 997.0,
                gravity: 9.8 * Vec3::NEG_Y,
            },
            FluidBounds,
            BoundaryConditions {
                x_min: FluidBoundaryMethod::Wall,
                x_max: FluidBoundaryMethod::Open,
                y_min: FluidBoundaryMethod::Open,
                y_max: FluidBoundaryMethod::Wall,
                z_min: FluidBoundaryMethod::Open,
                z_max: FluidBoundaryMethod::Open,
            },
            Transform::from_translation(Vec3::new(
                -0.75,
                fluid_half_size.y + source_fluid_half_size.y - 0.05,
                -fluid_half_size.z,
            )),
        ))
        .with_children(|commands| {
            commands.spawn((
                FluidSource {
                    avtive: true,
                    mode: FluidSourceMode::Source,
                },
                FluidSourceShape::Aabb {
                    half_size: Vec3::splat(0.08),
                },
                FluidSourceVelocity(Vec3::Z * 20.0),
                Transform::from_translation(Vec3::new(0.0, 0.2, -source_fluid_half_size.z * 0.5)),
            ));
        });

    let material_terrain = materials.add(Color::srgb(0.8, 0.8, 0.8));
    // 上面をy=0にする
    let floor_1_1 = Cuboid::new(2.0, 0.1, 2.0);
    commands.spawn((
        Name::new("Floor_1_1"),
        Mesh3d(meshes.add(floor_1_1)),
        MeshMaterial3d(material_terrain.clone()),
        floor_1_1.collider(),
        SolidShapeOnFluid::Cuboid(floor_1_1),
        Transform::default().with_translation(Vec3::new(0.0, -floor_1_1.half_size.y, 0.0)),
        RigidBody::Static,
    ));

    let slope_height = 0.3;
    let slope = Extrusion::<Triangle2d>::new(
        Triangle2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, slope_height),
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

    let floor_2_1 = Cuboid::new(2.0, slope_height, 0.5);
    commands.spawn((
        Name::new("Floor_2_1"),
        Mesh3d(meshes.add(floor_2_1)),
        MeshMaterial3d(material_terrain.clone()),
        floor_2_1.collider(),
        SolidShapeOnFluid::Cuboid(floor_2_1),
        Transform::default().with_translation(Vec3::new(
            0.0,
            -floor_2_1.half_size.y + slope_height,
            -0.75,
        )),
        RigidBody::Static,
    ));

    let floor_2_2 = Cuboid::new(0.5, 0.1, 2.0);
    commands.spawn((
        Name::new("Floor_2_2"),
        Mesh3d(meshes.add(floor_2_2)),
        MeshMaterial3d(material_terrain.clone()),
        floor_2_2.collider(),
        Transform::default().with_translation(Vec3::new(
            1.0 + floor_2_2.half_size.x,
            -floor_2_2.half_size.y + slope_height,
            0.0,
        )),
        RigidBody::Static,
    ));

    let slope_2 = Extrusion::<Triangle2d>::new(
        Triangle2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.3, 0.0),
            Vec2::new(0.3, slope_height),
        ),
        0.5,
    );
    let slope_2_mesh = meshes.add(slope_2);
    let slope_2_translate = Vec3::new(-1.0 + 0.3, slope_height, -0.75);
    commands.spawn((
        Name::new("Slope_2_1"),
        Mesh3d(slope_2_mesh.clone()),
        SolidShapeOnFluid::TriangularPrism(slope_2),
        TriangularPrism::from(slope_2).collider(),
        MeshMaterial3d(material_terrain.clone()),
        Transform::from_translation(slope_2_translate),
        RigidBody::Kinematic,
    ));
    commands.spawn((
        Name::new("Slope_2_2"),
        Mesh3d(slope_2_mesh),
        SolidShapeOnFluid::TriangularPrism(slope_2),
        TriangularPrism::from(slope_2).collider(),
        MeshMaterial3d(material_terrain.clone()),
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI))
            .with_translation(slope_2_translate),
        RigidBody::Kinematic,
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));

    let player_capsule = Capsule3d::new(0.05, 0.1);
    commands.spawn((
        Name::new("PlayerCapsule"),
        Transform::from_xyz(
            0.0,
            player_capsule.half_length + player_capsule.radius + 10.0,
            0.5,
        ),
        Mesh3d(meshes.add(player_capsule)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.0))),
        player_capsule.collider(),
        SolidShapeOnFluid::Capsule(player_capsule),
        CharacterController,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
    ));

    let moving_cube = Cuboid::from_size(Vec3::new(0.2, 0.2, 0.2));
    commands.spawn((
        Name::new("MovingCube"),
        Transform::default().with_translation(Vec3::new(-0.8, moving_cube.half_size.y, 0.0)),
        Mesh3d(meshes.add(moving_cube)),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        moving_cube.collider(),
        SolidShapeOnFluid::Cuboid(moving_cube),
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
