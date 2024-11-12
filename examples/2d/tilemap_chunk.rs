//! Demonstrates the use .

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let sprite_handle: Handle<Image> = asset_server.load("branding/icon.png");

    let mut texture_atlas_builder = TextureAtlasBuilder::default();


}
