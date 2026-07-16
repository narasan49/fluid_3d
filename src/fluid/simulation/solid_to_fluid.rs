use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::{
    material::descriptor::BindGroupLayoutDescriptor,
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, BindGroup, BindGroupLayoutEntries, PipelineCache, ShaderStages,
            ShaderType,
            binding_types::{storage_buffer, uniform_buffer},
        },
        renderer::RenderDevice,
        storage::{GpuShaderBuffer, ShaderBuffer},
        texture::{FallbackImage, GpuImage},
    },
};

pub struct SolidToFluidPlugin;

impl Plugin for SolidToFluidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<SolidBodyBuffer>::default())
            .add_systems(Startup, init_buffer)
            .add_systems(Update, update_solid_body_buffer);

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        let bind_group_layout = SolidBodyBufferBindGroupLayout(BindGroupLayoutDescriptor::new(
            "solid_body_buffer_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    storage_buffer::<Vec<SolidBody>>(false),
                    uniform_buffer::<u32>(false),
                ),
            ),
        ));

        render_app
            .add_systems(
                Render,
                prepare_bind_group.in_set(RenderSystems::PrepareBindGroups),
            )
            .insert_resource(bind_group_layout);
    }
}

// pub enum

#[derive(Resource, ExtractResource, Clone, AsBindGroup)]
pub struct SolidBodyBuffer {
    #[storage(0, read_only, visibility(compute))]
    pub solid_bodies: Handle<ShaderBuffer>,
    #[uniform(1)]
    pub length: u32,
}

#[derive(Resource)]
pub struct SolidBodyBufferBindGroupLayout(pub BindGroupLayoutDescriptor);

#[derive(Resource)]
pub struct SolidBodyBufferBindGroup(pub BindGroup);

#[derive(Component, Debug)]
pub enum SolidShapeOnFluid {
    Capsule(Capsule3d),
    Cuboid(Cuboid),
    TriangularPrism(Extrusion<Triangle2d>),
}

#[derive(ShaderType, Default)]
pub struct SolidBody {
    pub shape: ShapeVariant,
    pub linear_velocity: Vec3,
    pub transform: Mat4,
    pub inv_transform: Mat4,
}

#[derive(ShaderType, Default)]
pub struct ShapeVariant {
    pub shape_type: u32,
    pub values: [f32; 8],
}

impl From<&SolidShapeOnFluid> for ShapeVariant {
    fn from(value: &SolidShapeOnFluid) -> Self {
        match value {
            SolidShapeOnFluid::Capsule(capsule3d) => Self {
                shape_type: 1,
                values: [
                    capsule3d.radius,
                    capsule3d.half_length,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ],
            },
            SolidShapeOnFluid::Cuboid(cuboid) => Self {
                shape_type: 2,
                values: [
                    cuboid.half_size.x,
                    cuboid.half_size.y,
                    cuboid.half_size.z,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                ],
            },
            SolidShapeOnFluid::TriangularPrism(triangular_prism) => Self {
                shape_type: 3,
                values: [
                    triangular_prism.base_shape.vertices[0].x,
                    triangular_prism.base_shape.vertices[0].y,
                    triangular_prism.base_shape.vertices[1].x,
                    triangular_prism.base_shape.vertices[1].y,
                    triangular_prism.base_shape.vertices[2].x,
                    triangular_prism.base_shape.vertices[2].y,
                    triangular_prism.half_depth,
                    0.0,
                ],
            },
        }
    }
}

fn init_buffer(mut commands: Commands, mut buffers: ResMut<Assets<ShaderBuffer>>) {
    let buffer = buffers.add(ShaderBuffer::from([SolidBody::default(); 1]));
    let solid_body = SolidBodyBuffer {
        solid_bodies: buffer,
        length: 0,
    };
    commands.insert_resource(solid_body);
}

fn update_solid_body_buffer(
    query: Query<(&GlobalTransform, &SolidShapeOnFluid, &LinearVelocity)>,
    mut solid_body_buffer: ResMut<SolidBodyBuffer>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
) {
    let solid_bodies = query
        .iter()
        .map(|(transform, shape, velocity)| {
            let transform = transform.to_matrix();
            let inv_transform = transform.inverse();
            SolidBody {
                shape: shape.into(),
                linear_velocity: velocity.0,
                transform: transform,
                inv_transform,
            }
        })
        .collect::<Vec<_>>();

    solid_body_buffer.length = solid_bodies.len() as u32;
    let mut buffer = buffers.get_mut(&solid_body_buffer.solid_bodies).unwrap();
    buffer.set_data(solid_bodies);
}

fn prepare_bind_group<'a>(
    mut commands: Commands,
    solid_body_buffer: Res<SolidBodyBuffer>,
    bind_group_layout: Res<SolidBodyBufferBindGroupLayout>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    mut param: (
        Res<'a, RenderAssets<GpuImage>>,
        Res<'a, FallbackImage>,
        Res<'a, RenderAssets<GpuShaderBuffer>>,
    ),
) {
    let bind_group = solid_body_buffer
        .as_bind_group(
            &bind_group_layout.0,
            &render_device,
            &pipeline_cache,
            &mut param,
        )
        .unwrap()
        .bind_group;

    commands.insert_resource(SolidBodyBufferBindGroup(bind_group));
}
