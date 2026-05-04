use bevy::{post_process::bloom::Bloom, prelude::*};
use avian2d::prelude::*;

pub struct GamePlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
            MainCamera,
            Camera2d,
            Bloom::NATURAL,
    ));

    // Player
    commands.spawn((Player,
        Sprite::from_color(
        Srgba{ red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0}, 
        Vec2{ x: 50.0, y: 50.0 }
        )
    ));

    // Ground
    commands.spawn(Sprite::from_color(
        Srgba{ red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }, 
        Vec2{ x: 150.0, y: 30.0})
    );
}

fn update_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>
) {
    // Input
    const SPEED: f32 = 400.0;
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if keyboard_input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }

        // CHANGED: `Transform`
        transform.translation += direction * SPEED * time.delta_secs();
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<MainCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else { return };
    let Ok(mut camera_transform) = camera_query.single_mut() else { return };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    camera_transform.translation.z = player_transform.translation.z + 999.0;
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_player, camera_follow));
    }
}
