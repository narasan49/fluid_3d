use bevy::{prelude::*, render::storage::ShaderBuffer};

use crate::{
    fluid::simulation::solid_to_fluid::{ShapeVariant, SolidBody},
    marching_cubes::{MarchingCubes, shapes_to_sdf::ShapesToSdfResource},
};

#[derive(Component)]
pub struct SdfShape {
    pub shape: ShapeVariant,
}

pub trait IntoSdfShape {
    fn sdf_shape(&self) -> SdfShape;
}

impl IntoSdfShape for Capsule3d {
    fn sdf_shape(&self) -> SdfShape {
        SdfShape {
            shape: ShapeVariant {
                shape_type: 1,
                values: [self.radius, self.half_length, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            },
        }
    }
}

impl IntoSdfShape for Cuboid {
    fn sdf_shape(&self) -> SdfShape {
        SdfShape {
            shape: ShapeVariant {
                shape_type: 2,
                values: [
                    self.half_size.x,
                    self.half_size.y,
                    self.half_size.z,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ],
            },
        }
    }
}

pub(super) fn update_shapes_buffer(
    query: Query<(&ShapesToSdfResource, &Children), With<MarchingCubes>>,
    q_shapes: Query<(&SdfShape, &GlobalTransform)>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
) {
    for (resource, children) in &query {
        let mut shapes = Vec::with_capacity(children.len());
        // MarchingCubesの子のsdfshapeを取得したい
        for child in children {
            let Ok((shape, transform)) = q_shapes.get(*child) else {
                continue;
            };
            let transform = transform.to_matrix();
            let inv_transform = transform.inverse();

            shapes.push(SolidBody {
                shape: shape.shape,
                linear_velocity: Vec3::ZERO,
                angular_velocity: Vec3::ZERO,
                transform,
                inv_transform,
            });
        }

        let mut buffer = buffers.get_mut(&resource.shapes).unwrap();
        buffer.set_data(shapes);
    }
}
