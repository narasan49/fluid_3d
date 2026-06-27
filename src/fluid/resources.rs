use bevy::{
    asset::RenderAssetUsages,
    image::TextureFormatPixelInfo,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

#[derive(Component)]
pub struct FluidResources {
    pub levelset_air0: Handle<Image>,
}

impl FluidResources {
    pub fn new(images: &mut Assets<Image>, resolution: UVec3) -> Self {
        let levelset_air0 = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);

        Self { levelset_air0 }
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
