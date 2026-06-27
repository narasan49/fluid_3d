use bevy::math::UVec3;

pub const WORKGROUP_SIZE: UVec3 = UVec3::new(8, 8, 4);

pub fn num_workgroups(resolution: UVec3, workgroup_size: UVec3) -> UVec3 {
    (resolution + workgroup_size - 1) / workgroup_size
}
