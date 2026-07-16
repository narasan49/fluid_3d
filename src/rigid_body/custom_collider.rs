use avian3d::collision::collider::{Collider, IntoCollider};
use bevy::math::{
    Vec2,
    primitives::{Extrusion, Triangle2d},
};

pub struct TriangularPrism {
    pub triangle: [Vec2; 3],
    pub half_depth: f32,
}

impl From<Extrusion<Triangle2d>> for TriangularPrism {
    fn from(value: Extrusion<Triangle2d>) -> Self {
        Self {
            triangle: value.base_shape.vertices,
            half_depth: value.half_depth,
        }
    }
}

impl IntoCollider<Collider> for TriangularPrism {
    fn collider(&self) -> Collider {
        let vertices_front = self
            .triangle
            .map(|vertex_2d| vertex_2d.extend(-self.half_depth));
        let vertices_back = self
            .triangle
            .map(|vertex_2d| vertex_2d.extend(self.half_depth));
        let mut vertices = Vec::with_capacity(6);
        for i in 0..3 {
            vertices.push(vertices_front[i]);
        }
        for i in 0..3 {
            vertices.push(vertices_back[i]);
        }

        let indices = vec![
            [0, 1, 2],
            [0, 3, 4],
            [1, 0, 4],
            [1, 4, 5],
            [2, 1, 5],
            [2, 5, 3],
            [0, 2, 3],
            [3, 5, 4],
        ];

        Collider::trimesh(vertices, indices)
    }
}
