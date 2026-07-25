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
    diagnostics::fluid_bounds::FluidBoundsPlugin,
    fluid::{
        Fluid3d, Fluid3dPlugin, GridLength,
        resources::FluidResources,
        simulation::{fluid_source::FluidSource, solid_to_fluid::SolidShapeOnFluid},
    },
    game::{
        character_controller::{CharacterController, Player},
        input_mode::InputMode,
        solid_body_motion::{MovingObject, update_moving_object},
    },
    marching_cubes::{MarchingCubes, MarchingCubesPlugin},
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
        .add_systems(
            Update,
            (
                update_moving_object,
                toggle_fluid_source,
                toggle_free_camera,
            ),
        )
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
    let player_capsule = Capsule3d::new(0.05, 0.1);
    let moving_cube = Cuboid::from_size(Vec3::new(0.1, 0.2, 0.6));
    commands.spawn((
        game::scene::SceneRoot,
        children![
            game::scene::static_objects(&mut meshes, &mut materials),
            game::scene::fluids(grid_length.0, &mut meshes, &mut materials),
            (
                Name::new("Light"),
                PointLight::default(),
                Transform::from_xyz(0.0, 15.0, 0.0),
            ),
            (
                Name::new("PlayerCapsule"),
                Player,
                Transform::default()
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2))
                    .with_translation(Vec3::new(1.0, 0.5, 0.0,)),
                Mesh3d(meshes.add(player_capsule)),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.0))),
                player_capsule.collider(),
                SolidShapeOnFluid::Capsule(player_capsule),
                CharacterController::default(),
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                children![(
                    Camera3d::default(),
                    Camera {
                        order: 1,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.4, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
                )]
            ),
            (
                Name::new("MovingCube"),
                Transform::default().with_translation(Vec3::new(
                    -0.85,
                    moving_cube.half_size.y,
                    0.0
                )),
                Mesh3d(meshes.add(moving_cube)),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
                moving_cube.collider(),
                SolidShapeOnFluid::Cuboid(moving_cube),
                RigidBody::Kinematic,
                MovingObject,
            )
        ],
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

fn toggle_fluid_source(mut query: Query<&mut FluidSource>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyF) {
        for mut fluid_source in &mut query {
            fluid_source.active = !fluid_source.active;
        }
    }
}

fn toggle_free_camera(
    mut q_free: Query<(&mut FreeCameraState, &mut Camera), With<FreeCamera>>,
    mut q_player_camera: Query<&mut Camera, Without<FreeCamera>>,
    mut q_player: Query<&mut CharacterController>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        let (mut free_camera_state, mut free_camera) = q_free.single_mut().unwrap();
        free_camera_state.enabled = !free_camera_state.enabled;
        free_camera.is_active = free_camera_state.enabled;
        free_camera.order = if free_camera_state.enabled { 1 } else { 0 };
        let mut player_camera = q_player_camera.single_mut().unwrap();
        player_camera.is_active = !free_camera_state.enabled;
        player_camera.order = if free_camera_state.enabled { 0 } else { 1 };
        for mut character_controller in &mut q_player {
            character_controller.enabled = !free_camera_state.enabled;
        }
    }
}
