use bevy::{core_pipeline::core_2d::Transparent2d, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Player;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("crab.png"),
        transform: Transform::from_scale(Vec3 { x: 0.05, y: 0.05, z: 1.0 }),
        ..default()
    });
}

