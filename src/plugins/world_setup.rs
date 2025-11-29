use bevy::prelude::*;

pub struct WorldSetup;

fn setup_main(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 1.0, 0.3))),
    ));
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));
}

impl Plugin for WorldSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main);
    }
}
