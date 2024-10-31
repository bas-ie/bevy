use bevy_asset::Handle;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_reflect::std_traits::ReflectDefault;

use crate::TextureAtlasLayout;

#[derive(Default, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
pub struct TilemapChunk {
    /// Texture
    pub layout: Handle<TextureAtlasLayout>,

    pub size: Vec2,
}