
use bevy_asset::{Asset, Handle};
use bevy_color::Color;
use bevy_reflect::Reflect;
use bevy_render::{render_resource::AsBindGroup, texture::Image};

use crate::ColorMaterialUniform;

use super::{AlphaMode2d, MeshMaterial2d};




#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
#[uniform(0, ColorMaterialUniform)]
pub struct TilemapMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}
