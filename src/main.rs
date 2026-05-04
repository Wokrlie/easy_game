use bevy::{post_process::bloom::Bloom, prelude::*};

pub struct GamePlugin;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
            Camera2d,
            Bloom::NATURAL,
    ));

    // Player
    commands.spawn((Player,
        Sprite::from_color(
        Srgba{ red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0}, 
        Vec2{ x: 10.0, y:10.0})
    ));
}

fn update_player() {}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_player);
    }
}
