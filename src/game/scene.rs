use avian3d::{collision::collider::IntoCollider, dynamics::rigid_body::RigidBody};
use bevy::prelude::*;

use crate::{
    diagnostics::fluid_bounds::FluidBounds,
    fluid::{
        BoundaryConditions, Fluid3d, FluidBoundaryMethod,
        simulation::{
            fluid_source::{FluidSource, FluidSourceMode, FluidSourceShape, FluidSourceVelocity},
            solid_to_fluid::SolidShapeOnFluid,
        },
    },
    rigid_body::custom_collider::TriangularPrism,
};

#[derive(Component)]
#[require(Visibility, Transform)]
pub struct SceneRoot;

pub fn static_objects(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    let material_terrain = materials.add(Color::srgb(0.8, 0.8, 0.8));

    let floor_height = 0.3;

    let floor_1_1 = Cuboid::new(2.0, 0.1, 2.0);

    let slope_1 = Extrusion::<Triangle2d>::new(
        Triangle2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, floor_height),
        ),
        1.0,
    );

    let floor_2_1 = Cuboid::new(2.0, floor_height, 0.5);
    let floor_2_2 = Cuboid::new(0.5, 0.1, 2.0);
    let slope_2 = Extrusion::<Triangle2d>::new(
        Triangle2d::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.3, 0.0),
            Vec2::new(0.3, floor_height),
        ),
        0.5,
    );
    let slope_2_mesh = meshes.add(slope_2);
    let slope_2_translate = Vec3::new(-0.75, floor_height, -0.75);
    (
        Name::new("StaticObjectsRoot"),
        Visibility::Inherited,
        Transform::default(),
        children![
            (
                Name::new("Floor_1_1"),
                Mesh3d(meshes.add(floor_1_1)),
                MeshMaterial3d(material_terrain.clone()),
                floor_1_1.collider(),
                SolidShapeOnFluid::Cuboid(floor_1_1),
                Transform::default().with_translation(Vec3::new(0.0, -floor_1_1.half_size.y, 0.0)),
                RigidBody::Static,
            ),
            (
                Name::new("Slope_1"),
                Mesh3d(meshes.add(slope_1)),
                SolidShapeOnFluid::TriangularPrism(slope_1),
                TriangularPrism::from(slope_1).collider(),
                MeshMaterial3d(material_terrain.clone()),
                RigidBody::Static,
            ),
            (
                Name::new("Floor_2_1"),
                Mesh3d(meshes.add(floor_2_1)),
                MeshMaterial3d(material_terrain.clone()),
                floor_2_1.collider(),
                SolidShapeOnFluid::Cuboid(floor_2_1),
                Transform::default().with_translation(Vec3::new(
                    0.0,
                    -floor_2_1.half_size.y + floor_height,
                    -0.75,
                )),
                RigidBody::Static,
            ),
            (
                Name::new("Floor_2_2"),
                Mesh3d(meshes.add(floor_2_2)),
                MeshMaterial3d(material_terrain.clone()),
                floor_2_2.collider(),
                Transform::default().with_translation(Vec3::new(
                    1.0 + floor_2_2.half_size.x,
                    -floor_2_2.half_size.y + floor_height,
                    0.0,
                )),
                RigidBody::Static,
            ),
            (
                Name::new("Floor_2_3"),
                Mesh3d(meshes.add(floor_2_1)),
                MeshMaterial3d(material_terrain.clone()),
                floor_2_1.collider(),
                SolidShapeOnFluid::Cuboid(floor_2_1),
                Transform::default().with_translation(Vec3::new(
                    0.0,
                    -floor_2_1.half_size.y + floor_height,
                    0.75,
                )),
                RigidBody::Static,
            ),
            (
                Name::new("Slope_2_1"),
                Mesh3d(slope_2_mesh.clone()),
                SolidShapeOnFluid::TriangularPrism(slope_2),
                TriangularPrism::from(slope_2).collider(),
                MeshMaterial3d(material_terrain.clone()),
                Transform::from_translation(slope_2_translate),
                RigidBody::Static,
            ),
            (
                Name::new("Slope_2_2"),
                Mesh3d(slope_2_mesh),
                SolidShapeOnFluid::TriangularPrism(slope_2),
                TriangularPrism::from(slope_2).collider(),
                MeshMaterial3d(material_terrain.clone()),
                Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI))
                    .with_translation(slope_2_translate),
                RigidBody::Static,
            )
        ],
    )
}

pub fn fluids(
    grid_length: f32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    let material_terrain = materials.add(Color::srgb(0.8, 0.8, 0.8));
    let resolution = UVec3::new(128, 18, 64);
    let fluid_half_size = 0.5 * resolution.as_vec3() * grid_length;

    let source_fluid_resolution = UVec3::new(16, 32, 32);
    let source_fluid_half_size = 0.5 * resolution.as_vec3() * grid_length;

    let cylinder = Cylinder::new(0.1, 0.4);
    (
        Name::new("FluidsRoot"),
        Transform::default(),
        Visibility::Inherited,
        children![
            (
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
            ),
            (
                Fluid3d {
                    resolution: source_fluid_resolution,
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
                    2.0 * fluid_half_size.y + source_fluid_half_size.y,
                    -1.2 * fluid_half_size.z,
                )),
                Visibility::Inherited,
                children![
                    (
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
                            -source_fluid_half_size.z * 0.4,
                        )),
                        Visibility::Inherited,
                    ),
                    (
                        Mesh3d(meshes.add(cylinder)),
                        MeshMaterial3d(material_terrain.clone()),
                        cylinder.collider(),
                        Transform::default()
                            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
                            .with_translation(Vec3::new(0.0, 0.0, -0.05)),
                    )
                ]
            )
        ],
    )
}
