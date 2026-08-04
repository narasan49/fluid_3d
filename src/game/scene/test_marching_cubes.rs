use bevy::{
    prelude::*,
    render::{render_resource::TextureFormat, storage::ShaderBuffer},
};

use crate::{
    fluid::{resources::new_texture_storage_3d, simulation::solid_to_fluid::SolidBody},
    game::scene::SceneRoot,
    marching_cubes::{
        MarchingCubes,
        shapes_to_sdf::{ShapesToSdfResource, update_shapes_buffer::IntoSdfShape},
    },
};

pub fn spawn_test_marching_cubes_scene(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    buffers: &mut Assets<ShaderBuffer>,
) {
    let resolution = UVec3::splat(64);
    let half_size = Vec3::splat(0.5);

    let capsule = Capsule3d::new(0.1, 0.2);
    let cube = Cuboid::from_size(Vec3::splat(0.1));
    let grad_sdf = new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
    let shapes = buffers.add(ShaderBuffer::from([SolidBody::default(); 1]));
    commands.spawn((
        SceneRoot,
        children![
            (
                Camera3d::default(),
                Transform::default()
                    .with_translation(Vec3::new(0.0, 0.0, 1.0))
                    .looking_at(Vec3::ZERO, Vec3::Y),
            ),
            (
                MarchingCubes {
                    grad_sdf: grad_sdf.clone(),
                    resolution,
                    half_size,
                },
                ShapesToSdfResource { grad_sdf, shapes },
                Transform::default(),
                children![
                    (cube.sdf_shape(), Transform::from_xyz(0.0, 0.0, -0.2),),
                    (capsule.sdf_shape(), Transform::from_xyz(-0.15, 0.0, 0.0),),
                    (capsule.sdf_shape(), Transform::default(),),
                    (capsule.sdf_shape(), Transform::from_xyz(0.05, 0.33, 0.0),),
                    (capsule.sdf_shape(), Transform::from_xyz(0.28, 0.33, 0.0),),
                    (capsule.sdf_shape(), Transform::from_xyz(0.15, 0.0, 0.0),),
                ]
            )
        ],
    ));
}
