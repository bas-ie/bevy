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

    // texture_atlas_builder.add_texture(image_id, texture);

    // commands.spawn(tilemapChunk)

    // commands.spawn(Sprite::from_image(sprite_handle.clone()));
    // commands.spawn((
    //     Sprite {
    //         image: sprite_handle.clone(),
    //         // Alpha channel of the color controls transparency.
    //         color: Color::srgba(0.0, 0.0, 1.0, 0.7),
    //         ..default()
    //     },
    //     Transform::from_xyz(100.0, 0.0, 0.0),
    // ));
    // commands.spawn((
    //     Sprite {
    //         image: sprite_handle,
    //         color: Color::srgba(0.0, 1.0, 0.0, 0.3),
    //         ..default()
    //     },
    //     Transform::from_xyz(200.0, 0.0, 0.0),
    // ));
}
