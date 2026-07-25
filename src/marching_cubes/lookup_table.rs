use std::fmt::Display;

use bevy::render::render_resource::ShaderType;

#[derive(ShaderType, Clone, Copy)]
pub struct Edge {
    pub vertices: [u32; 2],
}

#[derive(ShaderType, Clone, Copy)]
pub struct EdgeTriangle {
    pub edge: [Edge; 3],
}

impl EdgeTriangle {
    const INVALID: Self = EdgeTriangle::new([[0, 0], [0, 0], [0, 0]]);
    const fn new(slice: [[u32; 2]; 3]) -> Self {
        Self {
            edge: [
                Edge { vertices: slice[0] },
                Edge { vertices: slice[1] },
                Edge { vertices: slice[2] },
            ],
        }
    }
}

impl Display for EdgeTriangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}], [{}, {}], [{}, {}]\n",
            self.edge[0].vertices[0],
            self.edge[0].vertices[1],
            self.edge[1].vertices[0],
            self.edge[1].vertices[1],
            self.edge[2].vertices[0],
            self.edge[2].vertices[1]
        )
    }
}

#[derive(ShaderType, Clone, Copy)]
pub struct EdgeTriangles {
    pub triangles: [EdgeTriangle; 5],
    pub count: u32,
}

impl Display for EdgeTriangles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}",
            self.triangles[0],
            self.triangles[1],
            self.triangles[2],
            self.triangles[3],
            self.triangles[4]
        )
    }
}

impl EdgeTriangles {
    const ZERO: Self = EdgeTriangles {
        triangles: [EdgeTriangle::INVALID; 5],
        count: 0,
    };

    const fn one(triangles: [EdgeTriangle; 1]) -> Self {
        Self {
            triangles: [
                triangles[0],
                EdgeTriangle::INVALID,
                EdgeTriangle::INVALID,
                EdgeTriangle::INVALID,
                EdgeTriangle::INVALID,
            ],
            count: 1,
        }
    }

    const fn two(triangles: [EdgeTriangle; 2]) -> Self {
        Self {
            triangles: [
                triangles[0],
                triangles[1],
                EdgeTriangle::INVALID,
                EdgeTriangle::INVALID,
                EdgeTriangle::INVALID,
            ],
            count: 2,
        }
    }

    const fn three(triangles: [EdgeTriangle; 3]) -> Self {
        Self {
            triangles: [
                triangles[0],
                triangles[1],
                triangles[2],
                EdgeTriangle::INVALID,
                EdgeTriangle::INVALID,
            ],
            count: 3,
        }
    }

    const fn four(triangles: [EdgeTriangle; 4]) -> Self {
        Self {
            triangles: [
                triangles[0],
                triangles[1],
                triangles[2],
                triangles[3],
                EdgeTriangle::INVALID,
            ],
            count: 4,
        }
    }

    const fn five(triangles: [EdgeTriangle; 5]) -> Self {
        Self {
            triangles: [
                triangles[0],
                triangles[1],
                triangles[2],
                triangles[3],
                triangles[4],
            ],
            count: 5,
        }
    }
}
pub const LUT: [EdgeTriangles; 256] = [
    EdgeTriangles::ZERO,
    EdgeTriangles::one([EdgeTriangle::new([[0, 1], [0, 4], [0, 2]])]),
    EdgeTriangles::one([EdgeTriangle::new([[1, 3], [1, 5], [0, 1]])]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 2], [1, 5], [0, 4]]),
        EdgeTriangle::new([[1, 5], [0, 2], [1, 3]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[0, 2], [2, 6], [2, 3]])]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 3], [0, 4], [2, 6]]),
        EdgeTriangle::new([[0, 4], [2, 3], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 6], [0, 1], [0, 2]]),
        EdgeTriangle::new([[0, 1], [2, 6], [1, 5]]),
        EdgeTriangle::new([[2, 3], [1, 5], [2, 6]]),
        EdgeTriangle::new([[1, 5], [2, 3], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 3], [1, 3], [2, 6]]),
        EdgeTriangle::new([[2, 6], [1, 3], [1, 5]]),
        EdgeTriangle::new([[2, 6], [1, 5], [0, 4]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[2, 3], [3, 7], [1, 3]])]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 4], [1, 3], [0, 1]]),
        EdgeTriangle::new([[1, 3], [0, 4], [3, 7]]),
        EdgeTriangle::new([[0, 2], [3, 7], [0, 4]]),
        EdgeTriangle::new([[3, 7], [0, 2], [2, 3]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 1], [3, 7], [1, 5]]),
        EdgeTriangle::new([[3, 7], [0, 1], [2, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 2], [2, 3], [0, 4]]),
        EdgeTriangle::new([[0, 4], [2, 3], [3, 7]]),
        EdgeTriangle::new([[0, 4], [3, 7], [1, 5]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[1, 3], [2, 6], [3, 7]]),
        EdgeTriangle::new([[2, 6], [1, 3], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 3], [0, 1], [3, 7]]),
        EdgeTriangle::new([[3, 7], [0, 1], [0, 4]]),
        EdgeTriangle::new([[3, 7], [0, 4], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 1], [0, 2], [1, 5]]),
        EdgeTriangle::new([[1, 5], [0, 2], [2, 6]]),
        EdgeTriangle::new([[1, 5], [2, 6], [3, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 4], [2, 6], [1, 5]]),
        EdgeTriangle::new([[1, 5], [2, 6], [3, 7]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[4, 5], [4, 6], [0, 4]])]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 6], [0, 1], [4, 5]]),
        EdgeTriangle::new([[0, 1], [4, 6], [0, 2]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [1, 5], [4, 5]]),
        EdgeTriangle::new([[1, 5], [4, 6], [1, 3]]),
        EdgeTriangle::new([[0, 4], [1, 3], [4, 6]]),
        EdgeTriangle::new([[1, 3], [0, 4], [0, 1]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 5], [4, 5], [1, 3]]),
        EdgeTriangle::new([[1, 3], [4, 5], [4, 6]]),
        EdgeTriangle::new([[1, 3], [4, 6], [0, 2]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 3], [4, 6], [2, 6]]),
        EdgeTriangle::new([[4, 6], [2, 3], [4, 5]]),
        EdgeTriangle::new([[0, 2], [4, 5], [2, 3]]),
        EdgeTriangle::new([[4, 5], [0, 2], [0, 4]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 6], [2, 6], [4, 5]]),
        EdgeTriangle::new([[4, 5], [2, 6], [2, 3]]),
        EdgeTriangle::new([[4, 5], [2, 3], [0, 1]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[4, 6], [1, 5], [4, 5]]),
        EdgeTriangle::new([[4, 6], [2, 6], [1, 5]]),
        EdgeTriangle::new([[1, 3], [1, 5], [2, 6]]),
        EdgeTriangle::new([[1, 3], [2, 6], [2, 3]]),
        EdgeTriangle::new([[0, 2], [0, 4], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 3], [1, 5], [4, 5]]),
        EdgeTriangle::new([[1, 3], [4, 5], [2, 3]]),
        EdgeTriangle::new([[2, 3], [4, 5], [4, 6]]),
        EdgeTriangle::new([[2, 3], [4, 6], [2, 6]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 3], [3, 7], [1, 3]]),
        EdgeTriangle::new([[0, 4], [4, 5], [4, 6]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[4, 6], [0, 2], [2, 3]]),
        EdgeTriangle::new([[4, 6], [2, 3], [3, 7]]),
        EdgeTriangle::new([[4, 5], [4, 6], [3, 7]]),
        EdgeTriangle::new([[4, 5], [1, 3], [0, 1]]),
        EdgeTriangle::new([[4, 5], [3, 7], [1, 3]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[3, 7], [1, 5], [4, 5]]),
        EdgeTriangle::new([[3, 7], [4, 5], [4, 6]]),
        EdgeTriangle::new([[2, 3], [3, 7], [4, 6]]),
        EdgeTriangle::new([[2, 3], [0, 4], [0, 1]]),
        EdgeTriangle::new([[2, 3], [4, 6], [0, 4]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 3], [3, 7], [1, 5]]),
        EdgeTriangle::new([[2, 3], [1, 5], [0, 2]]),
        EdgeTriangle::new([[0, 2], [1, 5], [4, 5]]),
        EdgeTriangle::new([[0, 2], [4, 5], [0, 4]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[1, 3], [0, 2], [0, 4]]),
        EdgeTriangle::new([[1, 3], [0, 4], [4, 5]]),
        EdgeTriangle::new([[3, 7], [1, 3], [4, 5]]),
        EdgeTriangle::new([[3, 7], [4, 6], [2, 6]]),
        EdgeTriangle::new([[3, 7], [4, 5], [4, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [4, 5], [4, 6]]),
        EdgeTriangle::new([[0, 1], [4, 6], [2, 6]]),
        EdgeTriangle::new([[0, 1], [2, 6], [1, 3]]),
        EdgeTriangle::new([[1, 3], [2, 6], [3, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 2], [0, 4], [0, 1]]),
        EdgeTriangle::new([[3, 7], [4, 6], [2, 6]]),
        EdgeTriangle::new([[3, 7], [4, 5], [4, 6]]),
        EdgeTriangle::new([[3, 7], [1, 5], [4, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 6], [2, 6], [4, 5]]),
        EdgeTriangle::new([[2, 6], [1, 5], [4, 5]]),
        EdgeTriangle::new([[2, 6], [3, 7], [1, 5]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[5, 7], [4, 5], [1, 5]])]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 2], [4, 5], [0, 4]]),
        EdgeTriangle::new([[4, 5], [0, 2], [5, 7]]),
        EdgeTriangle::new([[0, 1], [5, 7], [0, 2]]),
        EdgeTriangle::new([[5, 7], [0, 1], [1, 5]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 5], [1, 3], [5, 7]]),
        EdgeTriangle::new([[1, 3], [4, 5], [0, 1]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 5], [0, 4], [5, 7]]),
        EdgeTriangle::new([[5, 7], [0, 4], [0, 2]]),
        EdgeTriangle::new([[5, 7], [0, 2], [1, 3]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 2], [2, 6], [2, 3]]),
        EdgeTriangle::new([[1, 5], [5, 7], [4, 5]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[2, 3], [0, 1], [1, 5]]),
        EdgeTriangle::new([[2, 3], [1, 5], [5, 7]]),
        EdgeTriangle::new([[2, 6], [2, 3], [5, 7]]),
        EdgeTriangle::new([[2, 6], [4, 5], [0, 4]]),
        EdgeTriangle::new([[2, 6], [5, 7], [4, 5]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[4, 5], [0, 1], [0, 2]]),
        EdgeTriangle::new([[4, 5], [0, 2], [2, 6]]),
        EdgeTriangle::new([[5, 7], [4, 5], [2, 6]]),
        EdgeTriangle::new([[5, 7], [2, 3], [1, 3]]),
        EdgeTriangle::new([[5, 7], [2, 6], [2, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 3], [5, 7], [4, 5]]),
        EdgeTriangle::new([[1, 3], [4, 5], [0, 4]]),
        EdgeTriangle::new([[1, 3], [0, 4], [2, 3]]),
        EdgeTriangle::new([[2, 3], [0, 4], [2, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [3, 7], [5, 7]]),
        EdgeTriangle::new([[3, 7], [4, 5], [2, 3]]),
        EdgeTriangle::new([[1, 5], [2, 3], [4, 5]]),
        EdgeTriangle::new([[2, 3], [1, 5], [1, 3]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[4, 5], [3, 7], [5, 7]]),
        EdgeTriangle::new([[4, 5], [0, 4], [3, 7]]),
        EdgeTriangle::new([[2, 3], [3, 7], [0, 4]]),
        EdgeTriangle::new([[2, 3], [0, 4], [0, 2]]),
        EdgeTriangle::new([[0, 1], [1, 5], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[3, 7], [5, 7], [2, 3]]),
        EdgeTriangle::new([[2, 3], [5, 7], [4, 5]]),
        EdgeTriangle::new([[2, 3], [4, 5], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 3], [3, 7], [5, 7]]),
        EdgeTriangle::new([[2, 3], [5, 7], [0, 2]]),
        EdgeTriangle::new([[0, 2], [5, 7], [4, 5]]),
        EdgeTriangle::new([[0, 2], [4, 5], [0, 4]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[2, 6], [3, 7], [5, 7]]),
        EdgeTriangle::new([[2, 6], [5, 7], [4, 5]]),
        EdgeTriangle::new([[0, 2], [2, 6], [4, 5]]),
        EdgeTriangle::new([[0, 2], [1, 5], [1, 3]]),
        EdgeTriangle::new([[0, 2], [4, 5], [1, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [1, 5], [1, 3]]),
        EdgeTriangle::new([[2, 6], [4, 5], [0, 4]]),
        EdgeTriangle::new([[2, 6], [5, 7], [4, 5]]),
        EdgeTriangle::new([[2, 6], [3, 7], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 2], [2, 6], [3, 7]]),
        EdgeTriangle::new([[0, 2], [3, 7], [0, 1]]),
        EdgeTriangle::new([[0, 1], [3, 7], [5, 7]]),
        EdgeTriangle::new([[0, 1], [5, 7], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 5], [0, 4], [5, 7]]),
        EdgeTriangle::new([[0, 4], [3, 7], [5, 7]]),
        EdgeTriangle::new([[0, 4], [2, 6], [3, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[5, 7], [0, 4], [1, 5]]),
        EdgeTriangle::new([[0, 4], [5, 7], [4, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 1], [1, 5], [0, 2]]),
        EdgeTriangle::new([[0, 2], [1, 5], [5, 7]]),
        EdgeTriangle::new([[0, 2], [5, 7], [4, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 4], [0, 1], [4, 6]]),
        EdgeTriangle::new([[4, 6], [0, 1], [1, 3]]),
        EdgeTriangle::new([[4, 6], [1, 3], [5, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 6], [0, 2], [5, 7]]),
        EdgeTriangle::new([[5, 7], [0, 2], [1, 3]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[5, 7], [4, 6], [2, 6]]),
        EdgeTriangle::new([[5, 7], [2, 6], [2, 3]]),
        EdgeTriangle::new([[1, 5], [5, 7], [2, 3]]),
        EdgeTriangle::new([[1, 5], [0, 2], [0, 4]]),
        EdgeTriangle::new([[1, 5], [2, 3], [0, 2]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 5], [5, 7], [4, 6]]),
        EdgeTriangle::new([[1, 5], [4, 6], [0, 1]]),
        EdgeTriangle::new([[0, 1], [4, 6], [2, 6]]),
        EdgeTriangle::new([[0, 1], [2, 6], [0, 2]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [0, 2], [0, 4]]),
        EdgeTriangle::new([[5, 7], [2, 3], [1, 3]]),
        EdgeTriangle::new([[5, 7], [2, 6], [2, 3]]),
        EdgeTriangle::new([[5, 7], [4, 6], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 3], [1, 3], [2, 6]]),
        EdgeTriangle::new([[1, 3], [4, 6], [2, 6]]),
        EdgeTriangle::new([[1, 3], [5, 7], [4, 6]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[0, 4], [1, 5], [1, 3]]),
        EdgeTriangle::new([[0, 4], [1, 3], [2, 3]]),
        EdgeTriangle::new([[4, 6], [0, 4], [2, 3]]),
        EdgeTriangle::new([[4, 6], [3, 7], [5, 7]]),
        EdgeTriangle::new([[4, 6], [2, 3], [3, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 5], [1, 3], [0, 1]]),
        EdgeTriangle::new([[4, 6], [3, 7], [5, 7]]),
        EdgeTriangle::new([[4, 6], [2, 3], [3, 7]]),
        EdgeTriangle::new([[4, 6], [0, 2], [2, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [4, 6], [0, 4]]),
        EdgeTriangle::new([[5, 7], [0, 4], [0, 1]]),
        EdgeTriangle::new([[5, 7], [0, 1], [3, 7]]),
        EdgeTriangle::new([[3, 7], [0, 1], [2, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[3, 7], [5, 7], [2, 3]]),
        EdgeTriangle::new([[5, 7], [0, 2], [2, 3]]),
        EdgeTriangle::new([[5, 7], [4, 6], [0, 2]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [4, 6], [2, 6]]),
        EdgeTriangle::new([[5, 7], [2, 6], [3, 7]]),
        EdgeTriangle::new([[1, 5], [0, 2], [0, 4]]),
        EdgeTriangle::new([[1, 5], [1, 3], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 6], [2, 6], [3, 7]]),
        EdgeTriangle::new([[4, 6], [3, 7], [5, 7]]),
        EdgeTriangle::new([[1, 3], [0, 1], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[3, 7], [5, 7], [4, 6]]),
        EdgeTriangle::new([[3, 7], [4, 6], [2, 6]]),
        EdgeTriangle::new([[0, 4], [0, 1], [0, 2]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 6], [2, 6], [3, 7]]),
        EdgeTriangle::new([[3, 7], [5, 7], [4, 6]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[4, 6], [6, 7], [2, 6]])]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [0, 4], [4, 6]]),
        EdgeTriangle::new([[0, 4], [6, 7], [0, 1]]),
        EdgeTriangle::new([[2, 6], [0, 1], [6, 7]]),
        EdgeTriangle::new([[0, 1], [2, 6], [0, 2]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[1, 3], [1, 5], [0, 1]]),
        EdgeTriangle::new([[2, 6], [4, 6], [6, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[1, 5], [0, 4], [4, 6]]),
        EdgeTriangle::new([[1, 5], [4, 6], [6, 7]]),
        EdgeTriangle::new([[1, 3], [1, 5], [6, 7]]),
        EdgeTriangle::new([[1, 3], [2, 6], [0, 2]]),
        EdgeTriangle::new([[1, 3], [6, 7], [2, 6]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[6, 7], [0, 2], [4, 6]]),
        EdgeTriangle::new([[0, 2], [6, 7], [2, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 4], [4, 6], [0, 1]]),
        EdgeTriangle::new([[0, 1], [4, 6], [6, 7]]),
        EdgeTriangle::new([[0, 1], [6, 7], [2, 3]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[6, 7], [2, 3], [1, 3]]),
        EdgeTriangle::new([[6, 7], [1, 3], [1, 5]]),
        EdgeTriangle::new([[4, 6], [6, 7], [1, 5]]),
        EdgeTriangle::new([[4, 6], [0, 1], [0, 2]]),
        EdgeTriangle::new([[4, 6], [1, 5], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 3], [1, 5], [0, 4]]),
        EdgeTriangle::new([[1, 3], [0, 4], [2, 3]]),
        EdgeTriangle::new([[2, 3], [0, 4], [4, 6]]),
        EdgeTriangle::new([[2, 3], [4, 6], [2, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 3], [6, 7], [3, 7]]),
        EdgeTriangle::new([[6, 7], [1, 3], [4, 6]]),
        EdgeTriangle::new([[2, 3], [4, 6], [1, 3]]),
        EdgeTriangle::new([[4, 6], [2, 3], [2, 6]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[6, 7], [0, 4], [4, 6]]),
        EdgeTriangle::new([[6, 7], [3, 7], [0, 4]]),
        EdgeTriangle::new([[0, 1], [0, 4], [3, 7]]),
        EdgeTriangle::new([[0, 1], [3, 7], [1, 3]]),
        EdgeTriangle::new([[2, 3], [2, 6], [0, 2]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[0, 1], [2, 3], [2, 6]]),
        EdgeTriangle::new([[0, 1], [2, 6], [4, 6]]),
        EdgeTriangle::new([[1, 5], [0, 1], [4, 6]]),
        EdgeTriangle::new([[1, 5], [6, 7], [3, 7]]),
        EdgeTriangle::new([[1, 5], [4, 6], [6, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 3], [2, 6], [0, 2]]),
        EdgeTriangle::new([[1, 5], [6, 7], [3, 7]]),
        EdgeTriangle::new([[1, 5], [4, 6], [6, 7]]),
        EdgeTriangle::new([[1, 5], [0, 4], [4, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[6, 7], [3, 7], [4, 6]]),
        EdgeTriangle::new([[4, 6], [3, 7], [1, 3]]),
        EdgeTriangle::new([[4, 6], [1, 3], [0, 2]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [0, 4], [4, 6]]),
        EdgeTriangle::new([[0, 1], [4, 6], [1, 3]]),
        EdgeTriangle::new([[1, 3], [4, 6], [6, 7]]),
        EdgeTriangle::new([[1, 3], [6, 7], [3, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 2], [4, 6], [6, 7]]),
        EdgeTriangle::new([[0, 2], [6, 7], [3, 7]]),
        EdgeTriangle::new([[0, 2], [3, 7], [0, 1]]),
        EdgeTriangle::new([[0, 1], [3, 7], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[6, 7], [3, 7], [4, 6]]),
        EdgeTriangle::new([[3, 7], [0, 4], [4, 6]]),
        EdgeTriangle::new([[3, 7], [1, 5], [0, 4]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 5], [2, 6], [0, 4]]),
        EdgeTriangle::new([[2, 6], [4, 5], [6, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 6], [0, 2], [6, 7]]),
        EdgeTriangle::new([[6, 7], [0, 2], [0, 1]]),
        EdgeTriangle::new([[6, 7], [0, 1], [4, 5]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[2, 6], [0, 4], [0, 1]]),
        EdgeTriangle::new([[2, 6], [0, 1], [1, 3]]),
        EdgeTriangle::new([[6, 7], [2, 6], [1, 3]]),
        EdgeTriangle::new([[6, 7], [1, 5], [4, 5]]),
        EdgeTriangle::new([[6, 7], [1, 3], [1, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [6, 7], [2, 6]]),
        EdgeTriangle::new([[4, 5], [2, 6], [0, 2]]),
        EdgeTriangle::new([[4, 5], [0, 2], [1, 5]]),
        EdgeTriangle::new([[1, 5], [0, 2], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 2], [0, 4], [2, 3]]),
        EdgeTriangle::new([[2, 3], [0, 4], [4, 5]]),
        EdgeTriangle::new([[2, 3], [4, 5], [6, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[6, 7], [2, 3], [4, 5]]),
        EdgeTriangle::new([[4, 5], [2, 3], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 4], [0, 1], [0, 2]]),
        EdgeTriangle::new([[6, 7], [1, 5], [4, 5]]),
        EdgeTriangle::new([[6, 7], [1, 3], [1, 5]]),
        EdgeTriangle::new([[6, 7], [2, 3], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 5], [4, 5], [1, 3]]),
        EdgeTriangle::new([[4, 5], [2, 3], [1, 3]]),
        EdgeTriangle::new([[4, 5], [6, 7], [2, 3]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[4, 5], [6, 7], [3, 7]]),
        EdgeTriangle::new([[4, 5], [3, 7], [1, 3]]),
        EdgeTriangle::new([[0, 4], [4, 5], [1, 3]]),
        EdgeTriangle::new([[0, 4], [2, 3], [2, 6]]),
        EdgeTriangle::new([[0, 4], [1, 3], [2, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 2], [2, 3], [2, 6]]),
        EdgeTriangle::new([[4, 5], [1, 3], [0, 1]]),
        EdgeTriangle::new([[4, 5], [3, 7], [1, 3]]),
        EdgeTriangle::new([[4, 5], [6, 7], [3, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [6, 7], [3, 7]]),
        EdgeTriangle::new([[4, 5], [3, 7], [1, 5]]),
        EdgeTriangle::new([[0, 4], [2, 3], [2, 6]]),
        EdgeTriangle::new([[0, 4], [0, 1], [2, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 5], [4, 5], [6, 7]]),
        EdgeTriangle::new([[1, 5], [6, 7], [3, 7]]),
        EdgeTriangle::new([[2, 6], [0, 2], [2, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 4], [4, 5], [6, 7]]),
        EdgeTriangle::new([[0, 4], [6, 7], [0, 2]]),
        EdgeTriangle::new([[0, 2], [6, 7], [3, 7]]),
        EdgeTriangle::new([[0, 2], [3, 7], [2, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 3], [0, 1], [3, 7]]),
        EdgeTriangle::new([[0, 1], [6, 7], [3, 7]]),
        EdgeTriangle::new([[0, 1], [4, 5], [6, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[6, 7], [3, 7], [1, 5]]),
        EdgeTriangle::new([[6, 7], [1, 5], [4, 5]]),
        EdgeTriangle::new([[0, 1], [0, 2], [0, 4]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[6, 7], [3, 7], [1, 5]]),
        EdgeTriangle::new([[1, 5], [4, 5], [6, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 5], [4, 6], [4, 5]]),
        EdgeTriangle::new([[4, 6], [1, 5], [2, 6]]),
        EdgeTriangle::new([[5, 7], [2, 6], [1, 5]]),
        EdgeTriangle::new([[2, 6], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[2, 6], [5, 7], [6, 7]]),
        EdgeTriangle::new([[2, 6], [0, 2], [5, 7]]),
        EdgeTriangle::new([[1, 5], [5, 7], [0, 2]]),
        EdgeTriangle::new([[1, 5], [0, 2], [0, 1]]),
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[1, 3], [5, 7], [6, 7]]),
        EdgeTriangle::new([[1, 3], [6, 7], [2, 6]]),
        EdgeTriangle::new([[0, 1], [1, 3], [2, 6]]),
        EdgeTriangle::new([[0, 1], [4, 6], [4, 5]]),
        EdgeTriangle::new([[0, 1], [2, 6], [4, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
        EdgeTriangle::new([[1, 3], [2, 6], [0, 2]]),
        EdgeTriangle::new([[1, 3], [6, 7], [2, 6]]),
        EdgeTriangle::new([[1, 3], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[0, 2], [4, 6], [4, 5]]),
        EdgeTriangle::new([[0, 2], [4, 5], [1, 5]]),
        EdgeTriangle::new([[2, 3], [0, 2], [1, 5]]),
        EdgeTriangle::new([[2, 3], [5, 7], [6, 7]]),
        EdgeTriangle::new([[2, 3], [1, 5], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [4, 5], [0, 4]]),
        EdgeTriangle::new([[2, 3], [5, 7], [6, 7]]),
        EdgeTriangle::new([[2, 3], [1, 5], [5, 7]]),
        EdgeTriangle::new([[2, 3], [0, 1], [1, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [2, 3], [1, 3]]),
        EdgeTriangle::new([[6, 7], [1, 3], [5, 7]]),
        EdgeTriangle::new([[4, 6], [0, 1], [0, 2]]),
        EdgeTriangle::new([[4, 6], [4, 5], [0, 1]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 3], [1, 3], [5, 7]]),
        EdgeTriangle::new([[2, 3], [5, 7], [6, 7]]),
        EdgeTriangle::new([[4, 5], [0, 4], [4, 6]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[1, 5], [4, 6], [4, 5]]),
        EdgeTriangle::new([[1, 5], [1, 3], [4, 6]]),
        EdgeTriangle::new([[2, 6], [4, 6], [1, 3]]),
        EdgeTriangle::new([[2, 6], [1, 3], [2, 3]]),
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [1, 5], [1, 3]]),
        EdgeTriangle::new([[0, 2], [2, 3], [2, 6]]),
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [6, 7], [3, 7]]),
        EdgeTriangle::new([[0, 1], [4, 6], [4, 5]]),
        EdgeTriangle::new([[0, 1], [2, 6], [4, 6]]),
        EdgeTriangle::new([[0, 1], [2, 3], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 3], [2, 6], [0, 2]]),
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
        EdgeTriangle::new([[0, 2], [1, 5], [1, 3]]),
        EdgeTriangle::new([[0, 2], [4, 5], [1, 5]]),
        EdgeTriangle::new([[0, 2], [4, 6], [4, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 1], [1, 5], [1, 3]]),
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 1], [0, 2], [4, 6]]),
        EdgeTriangle::new([[0, 1], [4, 6], [4, 5]]),
        EdgeTriangle::new([[6, 7], [3, 7], [5, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 5], [0, 4], [4, 6]]),
        EdgeTriangle::new([[5, 7], [6, 7], [3, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[5, 7], [6, 7], [1, 5]]),
        EdgeTriangle::new([[1, 5], [6, 7], [2, 6]]),
        EdgeTriangle::new([[1, 5], [2, 6], [0, 4]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [2, 6], [0, 2]]),
        EdgeTriangle::new([[6, 7], [0, 2], [5, 7]]),
        EdgeTriangle::new([[5, 7], [0, 2], [0, 1]]),
        EdgeTriangle::new([[5, 7], [0, 1], [1, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [2, 6], [0, 4]]),
        EdgeTriangle::new([[6, 7], [0, 4], [5, 7]]),
        EdgeTriangle::new([[5, 7], [0, 4], [0, 1]]),
        EdgeTriangle::new([[5, 7], [0, 1], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 6], [0, 2], [6, 7]]),
        EdgeTriangle::new([[0, 2], [5, 7], [6, 7]]),
        EdgeTriangle::new([[0, 2], [1, 3], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [2, 3], [0, 2]]),
        EdgeTriangle::new([[6, 7], [0, 2], [0, 4]]),
        EdgeTriangle::new([[6, 7], [0, 4], [5, 7]]),
        EdgeTriangle::new([[5, 7], [0, 4], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[5, 7], [6, 7], [1, 5]]),
        EdgeTriangle::new([[6, 7], [0, 1], [1, 5]]),
        EdgeTriangle::new([[6, 7], [2, 3], [0, 1]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[5, 7], [6, 7], [2, 3]]),
        EdgeTriangle::new([[5, 7], [2, 3], [1, 3]]),
        EdgeTriangle::new([[0, 2], [0, 4], [0, 1]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[5, 7], [6, 7], [2, 3]]),
        EdgeTriangle::new([[2, 3], [1, 3], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [3, 7], [5, 7]]),
        EdgeTriangle::new([[0, 4], [2, 3], [2, 6]]),
        EdgeTriangle::new([[0, 4], [1, 3], [2, 3]]),
        EdgeTriangle::new([[0, 4], [1, 5], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 5], [1, 3], [0, 1]]),
        EdgeTriangle::new([[5, 7], [6, 7], [3, 7]]),
        EdgeTriangle::new([[0, 2], [2, 3], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 4], [0, 1], [2, 3]]),
        EdgeTriangle::new([[0, 4], [2, 3], [2, 6]]),
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
        EdgeTriangle::new([[2, 3], [2, 6], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 2], [0, 4], [1, 5]]),
        EdgeTriangle::new([[0, 2], [1, 5], [1, 3]]),
        EdgeTriangle::new([[5, 7], [6, 7], [3, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[1, 3], [0, 1], [1, 5]]),
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 1], [0, 2], [0, 4]]),
        EdgeTriangle::new([[3, 7], [5, 7], [6, 7]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[6, 7], [3, 7], [5, 7]])]),
    EdgeTriangles::one([EdgeTriangle::new([[6, 7], [5, 7], [3, 7]])]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 1], [0, 4], [0, 2]]),
        EdgeTriangle::new([[3, 7], [6, 7], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [5, 7], [1, 5]]),
        EdgeTriangle::new([[5, 7], [0, 1], [6, 7]]),
        EdgeTriangle::new([[1, 3], [6, 7], [0, 1]]),
        EdgeTriangle::new([[6, 7], [1, 3], [3, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[0, 2], [1, 3], [3, 7]]),
        EdgeTriangle::new([[0, 2], [3, 7], [6, 7]]),
        EdgeTriangle::new([[0, 4], [0, 2], [6, 7]]),
        EdgeTriangle::new([[0, 4], [5, 7], [1, 5]]),
        EdgeTriangle::new([[0, 4], [6, 7], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [2, 6], [6, 7]]),
        EdgeTriangle::new([[2, 6], [5, 7], [0, 2]]),
        EdgeTriangle::new([[3, 7], [0, 2], [5, 7]]),
        EdgeTriangle::new([[0, 2], [3, 7], [2, 3]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[0, 4], [2, 6], [6, 7]]),
        EdgeTriangle::new([[0, 4], [6, 7], [5, 7]]),
        EdgeTriangle::new([[0, 1], [0, 4], [5, 7]]),
        EdgeTriangle::new([[0, 1], [3, 7], [2, 3]]),
        EdgeTriangle::new([[0, 1], [5, 7], [3, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[5, 7], [2, 6], [6, 7]]),
        EdgeTriangle::new([[5, 7], [1, 5], [2, 6]]),
        EdgeTriangle::new([[0, 2], [2, 6], [1, 5]]),
        EdgeTriangle::new([[0, 2], [1, 5], [0, 1]]),
        EdgeTriangle::new([[1, 3], [3, 7], [2, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 3], [3, 7], [2, 3]]),
        EdgeTriangle::new([[0, 4], [5, 7], [1, 5]]),
        EdgeTriangle::new([[0, 4], [6, 7], [5, 7]]),
        EdgeTriangle::new([[0, 4], [2, 6], [6, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[5, 7], [2, 3], [6, 7]]),
        EdgeTriangle::new([[2, 3], [5, 7], [1, 3]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[5, 7], [1, 3], [0, 1]]),
        EdgeTriangle::new([[5, 7], [0, 1], [0, 4]]),
        EdgeTriangle::new([[6, 7], [5, 7], [0, 4]]),
        EdgeTriangle::new([[6, 7], [0, 2], [2, 3]]),
        EdgeTriangle::new([[6, 7], [0, 4], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[5, 7], [1, 5], [6, 7]]),
        EdgeTriangle::new([[6, 7], [1, 5], [0, 1]]),
        EdgeTriangle::new([[6, 7], [0, 1], [2, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 3], [6, 7], [5, 7]]),
        EdgeTriangle::new([[2, 3], [5, 7], [1, 5]]),
        EdgeTriangle::new([[2, 3], [1, 5], [0, 2]]),
        EdgeTriangle::new([[0, 2], [1, 5], [0, 4]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 6], [6, 7], [0, 2]]),
        EdgeTriangle::new([[0, 2], [6, 7], [5, 7]]),
        EdgeTriangle::new([[0, 2], [5, 7], [1, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [0, 4], [2, 6]]),
        EdgeTriangle::new([[0, 1], [2, 6], [1, 3]]),
        EdgeTriangle::new([[1, 3], [2, 6], [6, 7]]),
        EdgeTriangle::new([[1, 3], [6, 7], [3, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 2], [2, 6], [6, 7]]),
        EdgeTriangle::new([[0, 2], [6, 7], [0, 1]]),
        EdgeTriangle::new([[0, 1], [6, 7], [5, 7]]),
        EdgeTriangle::new([[0, 1], [5, 7], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[5, 7], [1, 5], [6, 7]]),
        EdgeTriangle::new([[1, 5], [2, 6], [6, 7]]),
        EdgeTriangle::new([[1, 5], [0, 4], [2, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 4], [6, 7], [4, 6]]),
        EdgeTriangle::new([[6, 7], [0, 4], [3, 7]]),
        EdgeTriangle::new([[4, 5], [3, 7], [0, 4]]),
        EdgeTriangle::new([[3, 7], [4, 5], [5, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[0, 1], [4, 5], [5, 7]]),
        EdgeTriangle::new([[0, 1], [5, 7], [3, 7]]),
        EdgeTriangle::new([[0, 2], [0, 1], [3, 7]]),
        EdgeTriangle::new([[0, 2], [6, 7], [4, 6]]),
        EdgeTriangle::new([[0, 2], [3, 7], [6, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[0, 4], [6, 7], [4, 6]]),
        EdgeTriangle::new([[0, 4], [0, 1], [6, 7]]),
        EdgeTriangle::new([[3, 7], [6, 7], [0, 1]]),
        EdgeTriangle::new([[3, 7], [0, 1], [1, 3]]),
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [5, 7], [1, 5]]),
        EdgeTriangle::new([[0, 2], [6, 7], [4, 6]]),
        EdgeTriangle::new([[0, 2], [3, 7], [6, 7]]),
        EdgeTriangle::new([[0, 2], [1, 3], [3, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[3, 7], [4, 5], [5, 7]]),
        EdgeTriangle::new([[3, 7], [2, 3], [4, 5]]),
        EdgeTriangle::new([[0, 4], [4, 5], [2, 3]]),
        EdgeTriangle::new([[0, 4], [2, 3], [0, 2]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
        EdgeTriangle::new([[0, 1], [3, 7], [2, 3]]),
        EdgeTriangle::new([[0, 1], [5, 7], [3, 7]]),
        EdgeTriangle::new([[0, 1], [4, 5], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 3], [3, 7], [2, 3]]),
        EdgeTriangle::new([[0, 1], [0, 2], [0, 4]]),
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 3], [3, 7], [2, 3]]),
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[2, 3], [6, 7], [4, 6]]),
        EdgeTriangle::new([[2, 3], [4, 6], [0, 4]]),
        EdgeTriangle::new([[1, 3], [2, 3], [0, 4]]),
        EdgeTriangle::new([[1, 3], [4, 5], [5, 7]]),
        EdgeTriangle::new([[1, 3], [0, 4], [4, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[0, 1], [4, 5], [5, 7]]),
        EdgeTriangle::new([[0, 1], [5, 7], [1, 3]]),
        EdgeTriangle::new([[0, 2], [6, 7], [4, 6]]),
        EdgeTriangle::new([[0, 2], [2, 3], [6, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
        EdgeTriangle::new([[2, 3], [0, 4], [0, 1]]),
        EdgeTriangle::new([[2, 3], [4, 6], [0, 4]]),
        EdgeTriangle::new([[2, 3], [6, 7], [4, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 2], [2, 3], [6, 7]]),
        EdgeTriangle::new([[0, 2], [6, 7], [4, 6]]),
        EdgeTriangle::new([[5, 7], [1, 5], [4, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [4, 6], [2, 6]]),
        EdgeTriangle::new([[1, 3], [4, 5], [5, 7]]),
        EdgeTriangle::new([[1, 3], [0, 4], [4, 5]]),
        EdgeTriangle::new([[1, 3], [0, 2], [0, 4]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 3], [0, 1], [4, 5]]),
        EdgeTriangle::new([[1, 3], [4, 5], [5, 7]]),
        EdgeTriangle::new([[4, 6], [2, 6], [6, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 2], [0, 4], [0, 1]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[5, 7], [1, 5], [4, 5]]),
        EdgeTriangle::new([[6, 7], [4, 6], [2, 6]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[6, 7], [1, 5], [3, 7]]),
        EdgeTriangle::new([[1, 5], [6, 7], [4, 5]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[6, 7], [4, 5], [0, 4]]),
        EdgeTriangle::new([[6, 7], [0, 4], [0, 2]]),
        EdgeTriangle::new([[3, 7], [6, 7], [0, 2]]),
        EdgeTriangle::new([[3, 7], [0, 1], [1, 5]]),
        EdgeTriangle::new([[3, 7], [0, 2], [0, 1]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 3], [3, 7], [0, 1]]),
        EdgeTriangle::new([[0, 1], [3, 7], [6, 7]]),
        EdgeTriangle::new([[0, 1], [6, 7], [4, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[3, 7], [6, 7], [4, 5]]),
        EdgeTriangle::new([[3, 7], [4, 5], [1, 3]]),
        EdgeTriangle::new([[1, 3], [4, 5], [0, 4]]),
        EdgeTriangle::new([[1, 3], [0, 4], [0, 1]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[1, 5], [3, 7], [2, 3]]),
        EdgeTriangle::new([[1, 5], [2, 3], [0, 2]]),
        EdgeTriangle::new([[4, 5], [1, 5], [0, 2]]),
        EdgeTriangle::new([[4, 5], [2, 6], [6, 7]]),
        EdgeTriangle::new([[4, 5], [0, 2], [2, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [4, 5], [0, 4]]),
        EdgeTriangle::new([[6, 7], [0, 4], [2, 6]]),
        EdgeTriangle::new([[3, 7], [0, 1], [1, 5]]),
        EdgeTriangle::new([[3, 7], [2, 3], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[3, 7], [2, 3], [1, 3]]),
        EdgeTriangle::new([[4, 5], [2, 6], [6, 7]]),
        EdgeTriangle::new([[4, 5], [0, 2], [2, 6]]),
        EdgeTriangle::new([[4, 5], [0, 1], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 5], [0, 4], [2, 6]]),
        EdgeTriangle::new([[4, 5], [2, 6], [6, 7]]),
        EdgeTriangle::new([[2, 3], [1, 3], [3, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 5], [1, 3], [4, 5]]),
        EdgeTriangle::new([[4, 5], [1, 3], [2, 3]]),
        EdgeTriangle::new([[4, 5], [2, 3], [6, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[1, 3], [0, 1], [1, 5]]),
        EdgeTriangle::new([[6, 7], [0, 2], [2, 3]]),
        EdgeTriangle::new([[6, 7], [0, 4], [0, 2]]),
        EdgeTriangle::new([[6, 7], [4, 5], [0, 4]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 5], [0, 1], [6, 7]]),
        EdgeTriangle::new([[6, 7], [0, 1], [2, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 2], [2, 3], [0, 4]]),
        EdgeTriangle::new([[2, 3], [4, 5], [0, 4]]),
        EdgeTriangle::new([[2, 3], [6, 7], [4, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[6, 7], [4, 5], [1, 5]]),
        EdgeTriangle::new([[6, 7], [1, 5], [1, 3]]),
        EdgeTriangle::new([[6, 7], [1, 3], [2, 6]]),
        EdgeTriangle::new([[2, 6], [1, 3], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 6], [6, 7], [4, 5]]),
        EdgeTriangle::new([[2, 6], [4, 5], [0, 4]]),
        EdgeTriangle::new([[1, 5], [1, 3], [0, 1]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 6], [6, 7], [0, 2]]),
        EdgeTriangle::new([[6, 7], [0, 1], [0, 2]]),
        EdgeTriangle::new([[6, 7], [4, 5], [0, 1]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 5], [0, 4], [2, 6]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[6, 7], [4, 6], [3, 7]]),
        EdgeTriangle::new([[3, 7], [4, 6], [0, 4]]),
        EdgeTriangle::new([[3, 7], [0, 4], [1, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [0, 2], [0, 1]]),
        EdgeTriangle::new([[4, 6], [0, 1], [1, 5]]),
        EdgeTriangle::new([[4, 6], [1, 5], [6, 7]]),
        EdgeTriangle::new([[6, 7], [1, 5], [3, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [0, 4], [0, 1]]),
        EdgeTriangle::new([[4, 6], [0, 1], [6, 7]]),
        EdgeTriangle::new([[6, 7], [0, 1], [1, 3]]),
        EdgeTriangle::new([[6, 7], [1, 3], [3, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[6, 7], [4, 6], [3, 7]]),
        EdgeTriangle::new([[4, 6], [1, 3], [3, 7]]),
        EdgeTriangle::new([[4, 6], [0, 2], [1, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [2, 6], [6, 7]]),
        EdgeTriangle::new([[1, 5], [0, 2], [0, 4]]),
        EdgeTriangle::new([[1, 5], [2, 3], [0, 2]]),
        EdgeTriangle::new([[1, 5], [3, 7], [2, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 1], [1, 5], [3, 7]]),
        EdgeTriangle::new([[0, 1], [3, 7], [2, 3]]),
        EdgeTriangle::new([[6, 7], [4, 6], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[3, 7], [2, 3], [1, 3]]),
        EdgeTriangle::new([[6, 7], [4, 6], [2, 6]]),
        EdgeTriangle::new([[0, 1], [0, 2], [0, 4]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 3], [1, 3], [3, 7]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [0, 4], [1, 5]]),
        EdgeTriangle::new([[4, 6], [1, 5], [6, 7]]),
        EdgeTriangle::new([[6, 7], [1, 5], [1, 3]]),
        EdgeTriangle::new([[6, 7], [1, 3], [3, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[6, 7], [4, 6], [0, 2]]),
        EdgeTriangle::new([[6, 7], [0, 2], [2, 3]]),
        EdgeTriangle::new([[0, 1], [1, 5], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 4], [0, 1], [4, 6]]),
        EdgeTriangle::new([[0, 1], [6, 7], [4, 6]]),
        EdgeTriangle::new([[0, 1], [2, 3], [6, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[6, 7], [4, 6], [0, 2]]),
        EdgeTriangle::new([[0, 2], [2, 3], [6, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 5], [1, 3], [0, 2]]),
        EdgeTriangle::new([[1, 5], [0, 2], [0, 4]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[1, 3], [0, 1], [1, 5]]),
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 6], [6, 7], [4, 6]]),
        EdgeTriangle::new([[0, 2], [0, 4], [0, 1]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[4, 6], [2, 6], [6, 7]])]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 6], [3, 7], [2, 6]]),
        EdgeTriangle::new([[3, 7], [4, 6], [5, 7]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[3, 7], [2, 6], [0, 2]]),
        EdgeTriangle::new([[3, 7], [0, 2], [0, 1]]),
        EdgeTriangle::new([[5, 7], [3, 7], [0, 1]]),
        EdgeTriangle::new([[5, 7], [0, 4], [4, 6]]),
        EdgeTriangle::new([[5, 7], [0, 1], [0, 4]]),
    ]),
    EdgeTriangles::five([
        EdgeTriangle::new([[4, 6], [5, 7], [1, 5]]),
        EdgeTriangle::new([[4, 6], [1, 5], [0, 1]]),
        EdgeTriangle::new([[2, 6], [4, 6], [0, 1]]),
        EdgeTriangle::new([[2, 6], [1, 3], [3, 7]]),
        EdgeTriangle::new([[2, 6], [0, 1], [1, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [5, 7], [1, 5]]),
        EdgeTriangle::new([[4, 6], [1, 5], [0, 4]]),
        EdgeTriangle::new([[2, 6], [1, 3], [3, 7]]),
        EdgeTriangle::new([[2, 6], [0, 2], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[3, 7], [2, 3], [5, 7]]),
        EdgeTriangle::new([[5, 7], [2, 3], [0, 2]]),
        EdgeTriangle::new([[5, 7], [0, 2], [4, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 6], [5, 7], [3, 7]]),
        EdgeTriangle::new([[4, 6], [3, 7], [2, 3]]),
        EdgeTriangle::new([[4, 6], [2, 3], [0, 4]]),
        EdgeTriangle::new([[0, 4], [2, 3], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 3], [1, 3], [3, 7]]),
        EdgeTriangle::new([[4, 6], [0, 1], [0, 2]]),
        EdgeTriangle::new([[4, 6], [1, 5], [0, 1]]),
        EdgeTriangle::new([[4, 6], [5, 7], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 4], [4, 6], [5, 7]]),
        EdgeTriangle::new([[0, 4], [5, 7], [1, 5]]),
        EdgeTriangle::new([[3, 7], [2, 3], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 3], [2, 6], [1, 3]]),
        EdgeTriangle::new([[1, 3], [2, 6], [4, 6]]),
        EdgeTriangle::new([[1, 3], [4, 6], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 6], [0, 2], [2, 3]]),
        EdgeTriangle::new([[5, 7], [0, 4], [4, 6]]),
        EdgeTriangle::new([[5, 7], [0, 1], [0, 4]]),
        EdgeTriangle::new([[5, 7], [1, 3], [0, 1]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[2, 6], [4, 6], [5, 7]]),
        EdgeTriangle::new([[2, 6], [5, 7], [2, 3]]),
        EdgeTriangle::new([[2, 3], [5, 7], [1, 5]]),
        EdgeTriangle::new([[2, 3], [1, 5], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[5, 7], [1, 5], [0, 4]]),
        EdgeTriangle::new([[5, 7], [0, 4], [4, 6]]),
        EdgeTriangle::new([[0, 2], [2, 3], [2, 6]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[5, 7], [1, 3], [4, 6]]),
        EdgeTriangle::new([[4, 6], [1, 3], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 4], [4, 6], [0, 1]]),
        EdgeTriangle::new([[4, 6], [1, 3], [0, 1]]),
        EdgeTriangle::new([[4, 6], [5, 7], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 1], [0, 2], [1, 5]]),
        EdgeTriangle::new([[0, 2], [5, 7], [1, 5]]),
        EdgeTriangle::new([[0, 2], [4, 6], [5, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[5, 7], [1, 5], [0, 4]]),
        EdgeTriangle::new([[0, 4], [4, 6], [5, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 5], [5, 7], [0, 4]]),
        EdgeTriangle::new([[0, 4], [5, 7], [3, 7]]),
        EdgeTriangle::new([[0, 4], [3, 7], [2, 6]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [3, 7], [2, 6]]),
        EdgeTriangle::new([[5, 7], [2, 6], [4, 5]]),
        EdgeTriangle::new([[4, 5], [2, 6], [0, 2]]),
        EdgeTriangle::new([[4, 5], [0, 2], [0, 4]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [1, 5], [4, 5]]),
        EdgeTriangle::new([[2, 6], [1, 3], [3, 7]]),
        EdgeTriangle::new([[2, 6], [0, 1], [1, 3]]),
        EdgeTriangle::new([[2, 6], [0, 4], [0, 1]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 6], [0, 2], [1, 3]]),
        EdgeTriangle::new([[2, 6], [1, 3], [3, 7]]),
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [3, 7], [2, 3]]),
        EdgeTriangle::new([[5, 7], [2, 3], [4, 5]]),
        EdgeTriangle::new([[4, 5], [2, 3], [0, 2]]),
        EdgeTriangle::new([[4, 5], [0, 2], [0, 4]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[3, 7], [2, 3], [5, 7]]),
        EdgeTriangle::new([[2, 3], [4, 5], [5, 7]]),
        EdgeTriangle::new([[2, 3], [0, 1], [4, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 4], [0, 1], [0, 2]]),
        EdgeTriangle::new([[4, 5], [5, 7], [1, 5]]),
        EdgeTriangle::new([[2, 3], [1, 3], [3, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
        EdgeTriangle::new([[1, 3], [3, 7], [2, 3]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[5, 7], [1, 3], [2, 3]]),
        EdgeTriangle::new([[5, 7], [2, 3], [2, 6]]),
        EdgeTriangle::new([[5, 7], [2, 6], [4, 5]]),
        EdgeTriangle::new([[4, 5], [2, 6], [0, 4]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 5], [5, 7], [1, 3]]),
        EdgeTriangle::new([[4, 5], [1, 3], [0, 1]]),
        EdgeTriangle::new([[2, 3], [2, 6], [0, 2]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 3], [2, 6], [0, 4]]),
        EdgeTriangle::new([[2, 3], [0, 4], [0, 1]]),
        EdgeTriangle::new([[4, 5], [5, 7], [1, 5]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 2], [2, 3], [2, 6]]),
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 5], [5, 7], [0, 4]]),
        EdgeTriangle::new([[5, 7], [0, 2], [0, 4]]),
        EdgeTriangle::new([[5, 7], [1, 3], [0, 2]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 5], [5, 7], [1, 3]]),
        EdgeTriangle::new([[1, 3], [0, 1], [4, 5]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 1], [0, 2], [0, 4]]),
        EdgeTriangle::new([[1, 5], [4, 5], [5, 7]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[5, 7], [1, 5], [4, 5]])]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 6], [4, 5], [2, 6]]),
        EdgeTriangle::new([[2, 6], [4, 5], [1, 5]]),
        EdgeTriangle::new([[2, 6], [1, 5], [3, 7]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [0, 4], [4, 6]]),
        EdgeTriangle::new([[3, 7], [0, 1], [1, 5]]),
        EdgeTriangle::new([[3, 7], [0, 2], [0, 1]]),
        EdgeTriangle::new([[3, 7], [2, 6], [0, 2]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [0, 1], [1, 3]]),
        EdgeTriangle::new([[4, 5], [1, 3], [3, 7]]),
        EdgeTriangle::new([[4, 5], [3, 7], [4, 6]]),
        EdgeTriangle::new([[4, 6], [3, 7], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 3], [3, 7], [2, 6]]),
        EdgeTriangle::new([[1, 3], [2, 6], [0, 2]]),
        EdgeTriangle::new([[4, 6], [4, 5], [0, 4]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [1, 5], [3, 7]]),
        EdgeTriangle::new([[4, 5], [3, 7], [4, 6]]),
        EdgeTriangle::new([[4, 6], [3, 7], [2, 3]]),
        EdgeTriangle::new([[4, 6], [2, 3], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[3, 7], [2, 3], [0, 1]]),
        EdgeTriangle::new([[3, 7], [0, 1], [1, 5]]),
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 6], [4, 5], [0, 1]]),
        EdgeTriangle::new([[4, 6], [0, 1], [0, 2]]),
        EdgeTriangle::new([[1, 3], [3, 7], [2, 3]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 3], [1, 3], [3, 7]]),
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
    ]),
    EdgeTriangles::four([
        EdgeTriangle::new([[4, 5], [1, 5], [1, 3]]),
        EdgeTriangle::new([[4, 5], [1, 3], [4, 6]]),
        EdgeTriangle::new([[4, 6], [1, 3], [2, 3]]),
        EdgeTriangle::new([[4, 6], [2, 3], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 6], [0, 2], [2, 3]]),
        EdgeTriangle::new([[4, 6], [4, 5], [0, 4]]),
        EdgeTriangle::new([[1, 3], [0, 1], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[4, 6], [4, 5], [2, 6]]),
        EdgeTriangle::new([[4, 5], [2, 3], [2, 6]]),
        EdgeTriangle::new([[4, 5], [0, 1], [2, 3]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 2], [2, 3], [2, 6]]),
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 5], [1, 3], [4, 5]]),
        EdgeTriangle::new([[1, 3], [4, 6], [4, 5]]),
        EdgeTriangle::new([[1, 3], [0, 2], [4, 6]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 4], [4, 6], [4, 5]]),
        EdgeTriangle::new([[0, 1], [1, 5], [1, 3]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[4, 6], [4, 5], [0, 1]]),
        EdgeTriangle::new([[0, 1], [0, 2], [4, 6]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[4, 5], [0, 4], [4, 6]])]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 6], [0, 4], [3, 7]]),
        EdgeTriangle::new([[3, 7], [0, 4], [1, 5]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 1], [1, 5], [0, 2]]),
        EdgeTriangle::new([[1, 5], [2, 6], [0, 2]]),
        EdgeTriangle::new([[1, 5], [3, 7], [2, 6]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[1, 3], [3, 7], [0, 1]]),
        EdgeTriangle::new([[3, 7], [0, 4], [0, 1]]),
        EdgeTriangle::new([[3, 7], [2, 6], [0, 4]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[1, 3], [3, 7], [2, 6]]),
        EdgeTriangle::new([[2, 6], [0, 2], [1, 3]]),
    ]),
    EdgeTriangles::three([
        EdgeTriangle::new([[0, 2], [0, 4], [2, 3]]),
        EdgeTriangle::new([[0, 4], [3, 7], [2, 3]]),
        EdgeTriangle::new([[0, 4], [1, 5], [3, 7]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 1], [1, 5], [3, 7]]),
        EdgeTriangle::new([[3, 7], [2, 3], [0, 1]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 2], [0, 4], [0, 1]]),
        EdgeTriangle::new([[2, 3], [1, 3], [3, 7]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[2, 3], [1, 3], [3, 7]])]),
    EdgeTriangles::three([
        EdgeTriangle::new([[2, 3], [2, 6], [1, 3]]),
        EdgeTriangle::new([[2, 6], [1, 5], [1, 3]]),
        EdgeTriangle::new([[2, 6], [0, 4], [1, 5]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 3], [2, 6], [0, 2]]),
        EdgeTriangle::new([[1, 3], [0, 1], [1, 5]]),
    ]),
    EdgeTriangles::two([
        EdgeTriangle::new([[2, 3], [2, 6], [0, 4]]),
        EdgeTriangle::new([[0, 4], [0, 1], [2, 3]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[0, 2], [2, 3], [2, 6]])]),
    EdgeTriangles::two([
        EdgeTriangle::new([[0, 2], [0, 4], [1, 5]]),
        EdgeTriangle::new([[1, 5], [1, 3], [0, 2]]),
    ]),
    EdgeTriangles::one([EdgeTriangle::new([[1, 3], [0, 1], [1, 5]])]),
    EdgeTriangles::one([EdgeTriangle::new([[0, 1], [0, 2], [0, 4]])]),
    EdgeTriangles::ZERO,
];
