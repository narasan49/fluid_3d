use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        render_resource::{BufferUsages, DrawIndirectArgs, ShaderType},
        storage::ShaderBuffer,
    },
};

use crate::marching_cubes::{
    MarchingCubes,
    build_vertex_buffer::BuildVertexBufferResource,
    draw::{MarchingCubesDrawResource, MarchingCubesUniform},
    lookup_table::LUT,
};

#[derive(ShaderType, Clone, Default)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
}

pub fn setup_marching_cubes_resources(
    mut commands: Commands,
    query: Query<(Entity, &MarchingCubes, &GlobalTransform), Added<MarchingCubes>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
) {
    for (entity, marching_cubes, transform) in &query {
        // 表面は2次元的なので、解像度の2/3乗に比例する頂点を確保
        let scale = (marching_cubes.resolution.element_product() as f32).powf(2.0 / 3.0) as u32;
        // 1キューブあたり最大5三角形 x 辺数
        let num_edges = 5 * 3;
        let num_vertices = 10 * scale * num_edges;
        let mut vertices = ShaderBuffer::from(vec![Vertex::default(); num_vertices as usize]);
        vertices.buffer_description.usage |=
            BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST;
        let vertices = buffers.add(vertices);

        let lookup_table = buffers.add(ShaderBuffer::from(LUT));
        let indirect_args = DrawIndirectArgs {
            vertex_count: 0,
            instance_count: 1,
            first_vertex: 0,
            first_instance: 0,
        };
        let mut indirect_args =
            ShaderBuffer::new(indirect_args.as_bytes(), RenderAssetUsages::default());
        indirect_args.buffer_description.usage |=
            BufferUsages::STORAGE | BufferUsages::INDIRECT | BufferUsages::COPY_DST;
        let indirect_args = buffers.add(indirect_args);

        let build_vertex_buffer_resource = BuildVertexBufferResource {
            vertices: vertices.clone(),
            indirect_args: indirect_args.clone(),
            lookup_table,
            sdf: marching_cubes.sdf.clone(),
            grad_sdf: marching_cubes.grad_sdf.clone(),
        };

        let marching_cubes_draw_resourcce = MarchingCubesDrawResource {
            vertices,
            indirect_args,
        };

        let marching_cubes_uniform = MarchingCubesUniform {
            world_from_local: transform.to_matrix(),
            half_size: marching_cubes.half_size,
        };

        commands.entity(entity).insert((
            build_vertex_buffer_resource,
            marching_cubes_draw_resourcce,
            marching_cubes_uniform,
        ));
    }
}
