use bevy::{
    asset::RenderAssetUsages,
    image::TextureFormatPixelInfo,
    prelude::*,
    render::{
        extract_component::ExtractComponent,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    },
};

#[derive(Component, ExtractComponent, Clone)]
pub struct FluidResources {
    /// 流体レベルセット(SDF)。0未満が流体、0以上がそれ以外を表す。
    pub levelset_air0: Handle<Image>,
    /// 流体レベルセットの中間バッファ
    pub levelset_air1: Handle<Image>,
    /// MarchingCubesの頂点生成用の流体レベルセットとその勾配。
    pub levelset_and_grad_air: Handle<Image>,
    /// 剛体レベルセット。0未満が剛体、0以上がそれ以外を表す。
    pub levelset_solid: Handle<Image>,
    /// ボクセルが流体を含むときの流体の速度
    pub u0: Handle<Image>,
    /// 流体速度の中間バッファ
    pub u1: Handle<Image>,
    /// MAC(Marker-And-Cell)グリッドにおける流体速度のx成分
    pub u_mac: Handle<Image>,
    /// MAC(Marker-And-Cell)グリッドにおける流体速度のy成分
    pub v_mac: Handle<Image>,
    /// MAC(Marker-And-Cell)グリッドにおける流体速度のz成分
    pub w_mac: Handle<Image>,
    /// ボクセルが固体を含むときの固体の速度
    pub u_solid: Handle<Image>,
    /// ボクセルの各面における、非固体が占める割合(area fraction)
    /// rgbチャンネルに-X, -Y, -Z面のarea fractionを格納する。
    /// サイズは resolution + UVec3::ONE
    pub non_solid_fraction: Handle<Image>,
    /// ボクセルの各面における、非流体が占める割合
    /// rgbチャンネルに-X, -Y, -Z面のarea fractionを格納する。
    /// サイズは resolution + UVec3::ONE
    pub non_fluid_fraction: Handle<Image>,
    /// ボクセルの発散場
    pub div: Handle<Image>,
    /// 流体の圧力
    pub p: Handle<Image>,
    /// FastIterativeMethodによるレベルセット再初期化に利用するラベル
    pub labels0: Handle<Image>,
    pub labels1: Handle<Image>,
    /// ExtrapolateVeocityで利用するラベル
    pub velocity_fixed: [Handle<Image>; 3],
    pub velocity_fixed1: Handle<Image>,
}

impl FluidResources {
    pub fn new(images: &mut Assets<Image>, resolution: UVec3, apron_width: u32) -> Self {
        let resolution_xyz = resolution + UVec3::ONE;
        let resolution_apron = resolution + UVec3::splat(2 * apron_width);
        let levelset_air0 =
            new_texture_storage_3d(images, resolution_apron, TextureFormat::R32Float);
        let levelset_air1 =
            new_texture_storage_3d(images, resolution_apron, TextureFormat::R32Float);
        let levelset_and_grad_air =
            new_texture_storage_3d(images, resolution, TextureFormat::Rgba16Float);
        let levelset_solid =
            new_texture_storage_3d(images, resolution_apron, TextureFormat::R32Float);

        let u0 = new_texture_storage_3d(images, resolution_apron, TextureFormat::Rgba16Float);
        let u1 = new_texture_storage_3d(images, resolution_apron, TextureFormat::Rgba16Float);
        let u_mac = new_texture_storage_3d(images, resolution + UVec3::X, TextureFormat::R16Float);
        let v_mac = new_texture_storage_3d(images, resolution + UVec3::Y, TextureFormat::R16Float);
        let w_mac = new_texture_storage_3d(images, resolution + UVec3::Z, TextureFormat::R16Float);
        let u_solid = new_texture_storage_3d(images, resolution_apron, TextureFormat::Rgba16Float);
        let non_solid_fraction =
            new_texture_storage_3d(images, resolution_xyz, TextureFormat::Rgba16Float);
        let non_fluid_fraction =
            new_texture_storage_3d(images, resolution_xyz, TextureFormat::Rgba16Float);
        let div = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);
        let p = new_texture_storage_3d(images, resolution, TextureFormat::R32Float);
        let labels0 = new_texture_storage_3d(images, resolution_apron, TextureFormat::R8Uint);
        let labels1 = new_texture_storage_3d(images, resolution_apron, TextureFormat::R8Uint);
        let velocity_fixed = [
            new_texture_storage_3d(images, resolution_xyz, TextureFormat::R8Uint),
            new_texture_storage_3d(images, resolution_xyz, TextureFormat::R8Uint),
            new_texture_storage_3d(images, resolution_xyz, TextureFormat::R8Uint),
        ];
        let velocity_fixed1 = new_texture_storage_3d(images, resolution_xyz, TextureFormat::R8Uint);

        Self {
            levelset_air0,
            levelset_air1,
            levelset_and_grad_air,
            levelset_solid,
            u0,
            u1,
            u_mac,
            v_mac,
            w_mac,
            u_solid,
            non_solid_fraction,
            non_fluid_fraction,
            div,
            p,
            labels0,
            labels1,
            velocity_fixed,
            velocity_fixed1,
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
