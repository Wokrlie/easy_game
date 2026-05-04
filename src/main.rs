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

    commands.spawn((Sprite, sprite: {
        color: Color::srgb(100, 100, 255),
        custom_size: Vec2(0, 0),
    })
}

impl Plugin for EntryPoint {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
