use std::array;

use bevy_asset::Handle;
use bevy_math::IVec2;
use bevy_reflect::Reflect;
use bevy_reflect::std_traits::ReflectDefault;

use crate::TextureAtlasLayout;

#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]

pub struct TilemapChunk {
    /// Texture
    pub layout: Handle<TextureAtlasLayout>,

    pub chunk_size: IVec2,
    // pub tile_ids: array<usize>,
}

// impl TilemapChunk {
//     fn from_
// }