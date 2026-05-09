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
        ),
        RigidBody::Dynamic,
        Collider::rectangle(50.0, 50.0),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Ground
    commands.spawn(Sprite::from_color(
        Srgba{ red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }, 
        Vec2{ x: 150.0, y: 30.0})
    );
}

fn update_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>
) {
    // Input
    let Ok(mut velocity) = query.single_mut() else { return };
    let speed = 400.0; 
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }

    direction = direction.normalize_or_zero();
    velocity.x = direction.x * speed;
    velocity.y = direction.y * speed;
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_query.single() else { return };
    let Ok(mut camera_tf) = camera_query.single_mut() else { return };

    let follow_speed = 8.0;
    let target = player_tf.translation + Vec3::new(0.0, 0.0, 999.0);
    
    let t = 1.0 - (-follow_speed * time.delta_secs()).exp();
    camera_tf.translation = camera_tf.translation.lerp(target, t);
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_player, camera_follow));
    }
}
