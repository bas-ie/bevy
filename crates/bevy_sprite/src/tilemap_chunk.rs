use std::array;

use bevy_asset::Handle;
use bevy_math::IVec2;
use bevy_reflect::Reflect;
use bevy_reflect::std_traits::ReflectDefault;
use bevy_render::texture::Image;

use crate::TextureAtlasLayout;

#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]

pub struct TilemapChunk {
    pub layout: Handle<TextureAtlasLayout>,
    pub tile_indicies: Image,
}

// impl TilemapChunk {
//     fn from_
// }