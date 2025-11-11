use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct Id {
    id: u32,
}

#[derive(Component)]
struct HitPoint {
    hp: i32,
}

#[derive(Component)]
struct Player;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d); // Bundleとかくとエラーになる
    commands.spawn((
        Id { id: 0 },
        HitPoint { hp: 100 },
        Player,
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
    ));
}

fn check_entety(query: Query<(&Id, &HitPoint)>) {
    for (id, hp) in &query {
        println!("entety ID : {} health : {}", id.id, hp.hp);
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, check_entety);
    }
}
