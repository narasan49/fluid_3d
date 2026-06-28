use bevy::{
    asset::RenderAssetUsages,
    image::TextureFormatPixelInfo,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

#[derive(Component)]
pub struct FluidResources {
    pub levelset_air0: Handle<Image>,
    pub levelset_air1: Handle<Image>,
    pub grad_levelset_air: Handle<Image>,
    pub u0: Handle<Image>,
    pub u1: Handle<Image>,
    pub u_solid: Handle<Image>,
    pub solid_fraction: Handle<Image>,
    pub div: Handle<Image>,
}

impl FluidResources {
    pub fn new(images: &mut Assets<Image>, resolution: UVec3) -> Self {
        let resolution_xyz = resolution + UVec3::ONE;
        let levelset_air0 = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);
        let levelset_air1 = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);
        let grad_levelset_air =
            new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Snorm);
        let u0 = new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
        let u1 = new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
        let u_solid = new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
        let solid_fraction =
            new_texture_storage_3d(images, resolution_xyz, TextureFormat::Rgba16Float);
        let div = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);

        Self {
            levelset_air0,
            levelset_air1,
            grad_levelset_air,
            u0,
            u1,
            u_solid,
            solid_fraction,
            div,
        }
    }
}

pub fn new_texture_storage_3d(
    images: &mut Assets<Image>,
    resolution: UVec3,
    format: TextureFormat,
) -> Handle<Image> {
    let pixel_size = format.pixel_size().unwrap();
    let zeros = vec![0u8; pixel_size];

    let mut image = Image::new_fill(
        Extent3d {
            width: resolution.x,
            height: resolution.y,
            depth_or_array_layers: resolution.z,
        },
        TextureDimension::D3,
        &zeros,
        format,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    images.add(image)
}
