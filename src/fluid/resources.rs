use bevy::{
    asset::RenderAssetUsages,
    image::TextureFormatPixelInfo,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

#[derive(Component)]
pub struct FluidResources {
    /// 流体レベルセット(SDF)。0未満が流体、0以上がそれ以外を表す。
    pub levelset_air0: Handle<Image>,
    /// 流体レベルセットの中間バッファ
    pub levelset_air1: Handle<Image>,
    /// 流体レベルセットの勾配
    pub grad_levelset_air: Handle<Image>,
    /// 剛体レベルセット。0未満が剛体、0以上がそれ以外を表す。
    pub levelset_solid: Handle<Image>,
    /// ボクセルが流体を含むときの流体の速度
    pub u0: Handle<Image>,
    /// 流体速度の中間バッファ
    pub u1: Handle<Image>,
    /// ボクセルが固体を含むときの固体の速度
    pub u_solid: Handle<Image>,
    /// ボクセルの各面における、固体に対して流体が占める割合(area fraction)。
    /// rgbチャンネルに-X, -Y, -Z面のarea fractionを格納する。
    /// サイズは resolution + UVec3::ONE
    pub fluid_fraction: Handle<Image>,
    /// ボクセルの発散場
    pub div: Handle<Image>,
}

impl FluidResources {
    pub fn new(images: &mut Assets<Image>, resolution: UVec3) -> Self {
        let resolution_xyz = resolution + UVec3::ONE;
        let levelset_air0 = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);
        let levelset_air1 = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);
        let grad_levelset_air =
            new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Snorm);
        let levelset_solid = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);

        let u0 = new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
        let u1 = new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
        let u_solid = new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
        let fluid_fraction =
            new_texture_storage_3d(images, resolution_xyz, TextureFormat::Rgba16Float);
        let div = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);

        Self {
            levelset_air0,
            levelset_air1,
            grad_levelset_air,
            levelset_solid,
            u0,
            u1,
            u_solid,
            fluid_fraction,
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
