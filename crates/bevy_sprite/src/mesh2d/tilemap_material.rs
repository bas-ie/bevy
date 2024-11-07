
use bevy_asset::{Asset, Handle};
use bevy_reflect::Reflect;
use bevy_reflect::prelude::ReflectDefault;
use bevy_render::{render_resource::AsBindGroup, texture::Image};





#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct TilemapMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl TilemapMaterial {

}

impl Default for TilemapMaterial {
    fn default() -> Self {
        TilemapMaterial {
            texture: None
        }
    }
}

impl From<Handle<Image>> for TilemapMaterial {
    fn from(texture: Handle<Image>) -> Self {
        TilemapMaterial {
            texture: Some(texture),
            ..Default::default()
        }
    }
}