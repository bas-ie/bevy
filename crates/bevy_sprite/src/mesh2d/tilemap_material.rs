
use bevy_app::{App, Plugin};
use bevy_asset::{load_internal_asset, Asset, AssetApp, Assets, Handle};
use bevy_reflect::{Array, Reflect};
use bevy_reflect::prelude::ReflectDefault;
use bevy_render::{render_resource::{AsBindGroup, GpuArrayBuffer, Shader, ShaderRef}, texture::Image};

use super::{Material2d, Material2dPlugin};

pub const TILEMAP_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(3778691234201231346);

pub struct TilemapMaterialPlugin;

impl Plugin for TilemapMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            TILEMAP_MATERIAL_SHADER_HANDLE,
            "color_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<TilemapMaterial>::default())
            .register_asset_reflect::<TilemapMaterial>();

        app.world_mut()
            .resource_mut::<Assets<TilemapMaterial>>()
            .insert(
                &Handle::<TilemapMaterial>::default(),
                TilemapMaterial::default(),
            );
    }
}

// wgpu way: 
// mapped_at_creation
// write_buffer for web gpu
// 
// wgpu::util::StagingBelt for everything else

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct TilemapMaterial {
    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,
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

impl Material2d for TilemapMaterial {
    fn fragment_shader() -> ShaderRef {
        TILEMAP_MATERIAL_SHADER_HANDLE.into()
    }
}