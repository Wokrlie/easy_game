use bevy::{post_process::bloom::Bloom, prelude::*};

pub struct EntryPoint;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntryPoint)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
            Camera2d,
            Bloom::NATURAL,
    ));

    // Player
    commands.spawn(Sprite::from_color(
        Srgba{ red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0}, 
        Vec2{ x: 10.0, y:10.0})
    );
}

impl Plugin for EntryPoint {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
