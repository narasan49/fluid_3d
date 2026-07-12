use avian3d::{
    collision::collider::Collider, dynamics::rigid_body::LinearVelocity, parry::shape::ShapeType,
};
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
    pub values: [f32; 4],
}

impl ShapeVariant {
    fn from_capsule_variant(capsule: &avian3d::parry::shape::Capsule) -> Self {
        // y-軸に沿ったCapsuleを前提にする。
        // ToDo: 一般のline segment
        let half_length = 0.5 * (capsule.segment.a.y - capsule.segment.b.y);
        Self {
            shape_type: 1,
            values: [half_length, capsule.radius, 0.0, 0.0],
        }
    }

    fn from_cuboid_variant(cuboid: &avian3d::parry::shape::Cuboid) -> Self {
        Self {
            shape_type: 2,
            values: [
                cuboid.half_extents.x,
                cuboid.half_extents.y,
                cuboid.half_extents.z,
                0.0,
            ],
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
    query: Query<(&GlobalTransform, &Collider, &LinearVelocity)>,
    mut solid_body_buffer: ResMut<SolidBodyBuffer>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
) {
    let solid_bodies = query
        .iter()
        .filter_map(|(transform, collider, velocity)| {
            let shape = collider.shape().shape_type();
            let transform = transform.to_matrix();
            let inv_transform = transform.inverse();
            match shape {
                ShapeType::Cuboid => {
                    let cuboid = collider.shape().as_cuboid().unwrap();
                    Some(SolidBody {
                        shape: ShapeVariant::from_cuboid_variant(cuboid),
                        linear_velocity: velocity.0,
                        transform: transform,
                        inv_transform,
                    })
                }
                ShapeType::Capsule => {
                    let capsule = collider.shape().as_capsule().unwrap();
                    Some(SolidBody {
                        shape: ShapeVariant::from_capsule_variant(capsule),
                        linear_velocity: velocity.0,
                        transform,
                        inv_transform,
                    })
                }
                _ => {
                    warn!("shape {:?} is not implemented", shape);
                    None
                }
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
