use std::time::Duration;

use avian2d::prelude::*;
use bevy::{post_process::bloom::Bloom, prelude::*};

pub struct GamePlugin;

pub enum PlayerAnimationState {
    Idle,
}

const MAX_PLAYER_ANIM_FRAME_STATE: u32 = 2; // 2 frames
#[derive(Component)]
pub struct Player {
    pub animation_frame_state: u32,
    pub animation_state: PlayerAnimationState,
}

#[derive(Component)]
struct AnimationTimer(Timer);

// The warning of the AnimationFrames is wrong, the player animation texture is a reference of frames of this struct
#[allow(dead_code)]
#[derive(Component)]
struct AnimationFrames {
    frames: Vec<Handle<Image>>,
    current: usize,
}

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(GamePlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut idle_frames = Vec::new();
    for i in 0..MAX_PLAYER_ANIM_FRAME_STATE {
        let path = format!("idle-{}.png", i);
        idle_frames.push(asset_server.load(&path));
    }

    // Camera
    commands.spawn((MainCamera, Camera2d, Bloom::NATURAL));

    // Player
    commands.spawn((
        Player {
            animation_frame_state: 0,
            animation_state: PlayerAnimationState::Idle,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite::from_image(idle_frames[0].clone()),
        RigidBody::Dynamic,
        Collider::rectangle(50.0, 50.0),
        LockedAxes::ROTATION_LOCKED,
        AnimationTimer(Timer::new(
            Duration::from_secs_f32(0.5),
            TimerMode::Repeating,
        )),
        AnimationFrames {
            frames: idle_frames,
            current: 0,
        },
    ));

    // Ground
    commands.spawn(Sprite::from_color(
        Srgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        },
        Vec2 { x: 150.0, y: 30.0 },
    ));
}

fn update_player_movements(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut Player), With<Player>>,
) {
    let speed = 400.0;
    let mut direction = Vec2::ZERO;
    for (mut velocity, mut player) in &mut query {
        // Input
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        } else {
            player.animation_state = PlayerAnimationState::Idle;
        }

        // Movements
        direction = direction.normalize_or_zero();
        velocity.x = direction.x * speed;
        velocity.y = direction.y * speed;
    }
}

fn update_player_animation(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite, &mut Player), With<Player>>,
) {
    for (mut timer, mut sprite, mut player) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // Animation
            // We have to update the animation frame state at first.
            player.animation_frame_state =
                (player.animation_frame_state + 1) % MAX_PLAYER_ANIM_FRAME_STATE;
            // We get a string path of the assets. It is includes by assets folder.
            let state_name = match player.animation_state {
                PlayerAnimationState::Idle => "idle",
            };
            let path = format!("{}-{}.png", state_name, player.animation_frame_state);
            println!("[DEBUG]The current path is {}", path);
            sprite.image = asset_server.load(path);
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_query.single() else {
        return;
    };
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };

    let follow_speed = 8.0;
    let target = player_tf.translation + Vec3::new(0.0, 0.0, 999.0);

    let t = 1.0 - (-follow_speed * time.delta_secs()).exp();
    camera_tf.translation = camera_tf.translation.lerp(target, t);
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                update_player_movements,
                update_player_animation,
                camera_follow,
            ),
        );
    }
}
