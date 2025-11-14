use bevy::prelude::*;

pub struct WorldSetup;

fn setup_main(mut commands: Commands) {
    commands.spawn(PointLight {
        color: Color::srgb(1.0, 1.0, 1.0),
        intensity: 10.0,
        range: 100.0,
        radius: 3.0,
        ..default()
    });
}

impl Plugin for WorldSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main);
    }
}
