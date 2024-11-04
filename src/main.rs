use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("crab.png"),
            transform: Transform::from_scale(Vec3 {
                x: 0.05,
                y: 0.05,
                z: 1.0,
            }),
            ..default()
        },
        Player { speed: 200.0 },
    ));
}

fn sprite_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut sprite_position: Query<(&mut Player, &mut Transform)>,
) {
    let (player, mut transform) = sprite_position.single_mut();
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += player.speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= player.speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += player.speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= player.speed * time.delta_seconds();
    }
}
