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
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin, FreeCameraState},
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
        character_controller::{CharacterController, Player},
        input_mode::InputMode,
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
            FluidBoundsPlugin,
            game::GamePlugin,
        ))
        .add_systems(Startup, setup_dev_tools)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, setup_fluid_render)
        .add_systems(Update, (update_moving_object, toggle_free_camera))
        .insert_resource(Gravity(9.8 * Vec3::NEG_Y))
        .insert_resource(GridLength(1.0 / LENGTH_UNIT))
        .insert_resource(InputMode::Game)
        .run();
}

fn setup_dev_tools(mut commands: Commands) {
    let mut free_camera_state = FreeCameraState::default();
    free_camera_state.enabled = false;
    commands.spawn((
        Camera3d::default(),
        Camera {
            is_active: false,
            ..default()
        },
        Transform::from_xyz(0.0, 0.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
        free_camera_state,
    ));

    commands.spawn((InfiniteGrid, InfiniteGridSettings::default()));
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_length: Res<GridLength>,
) {
    let material_terrain = materials.add(Color::srgb(0.8, 0.8, 0.8));
    // 流体。底面をy=0に設定する。
    let resolution = UVec3::new(128, 18, 64);
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

    let resolution = UVec3::new(16, 32, 32);
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
                2.0 * fluid_half_size.y + source_fluid_half_size.y - 0.15,
                -fluid_half_size.z,
            )),
            Visibility::Inherited,
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    FluidSource {
                        active: true,
                        mode: FluidSourceMode::Source,
                    },
                    FluidSourceShape::Aabb {
                        half_size: Vec3::splat(0.06),
                    },
                    FluidSourceVelocity(Vec3::Z * 20.0),
                    Transform::from_translation(Vec3::new(
                        0.0,
                        -0.05,
                        -source_fluid_half_size.z * 0.8,
                    )),
                    Visibility::Inherited,
                ))
                .with_children(|commands| {
                    let cylinder = Cylinder::new(0.1, 0.4);
                    commands.spawn((
                        Mesh3d(meshes.add(cylinder)),
                        MeshMaterial3d(material_terrain.clone()),
                        cylinder.collider(),
                        Transform::default()
                            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                            .with_translation(Vec3::new(0.0, 0.0, -0.05)),
                    ));
                });
        });

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

    commands.spawn((
        Name::new("Floor_2_3"),
        Mesh3d(meshes.add(floor_2_1)),
        MeshMaterial3d(material_terrain.clone()),
        floor_2_1.collider(),
        SolidShapeOnFluid::Cuboid(floor_2_1),
        Transform::default().with_translation(Vec3::new(
            0.0,
            -floor_2_1.half_size.y + slope_height,
            0.75,
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
    let slope_2_translate = Vec3::new(-0.75, slope_height, -0.75);
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
    commands
        .spawn((
            Name::new("PlayerCapsule"),
            Player,
            Transform::from_xyz(
                1.0,
                player_capsule.half_length + player_capsule.radius + 10.0,
                0.0,
            ),
            Mesh3d(meshes.add(player_capsule)),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.0))),
            player_capsule.collider(),
            SolidShapeOnFluid::Capsule(player_capsule),
            CharacterController::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|commands| {
            commands.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });

    let moving_cube = Cuboid::from_size(Vec3::new(0.1, 0.2, 0.8));
    commands.spawn((
        Name::new("MovingCube"),
        Transform::default().with_translation(Vec3::new(-0.85, moving_cube.half_size.y, 0.0)),
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

fn toggle_free_camera(
    mut q_free: Query<(&mut FreeCameraState, &mut Camera), With<FreeCamera>>,
    mut q_player_camera: Query<&mut Camera, Without<FreeCamera>>,
    mut q_player: Query<&mut CharacterController>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        let (mut free_camera_state, mut camera) = q_free.single_mut().unwrap();
        free_camera_state.enabled = !free_camera_state.enabled;
        camera.is_active = free_camera_state.enabled;
        let mut player_camera = q_player_camera.single_mut().unwrap();
        player_camera.is_active = !free_camera_state.enabled;
        for mut character_controller in &mut q_player {
            character_controller.enabled = !free_camera_state.enabled;
        }
    }
}
