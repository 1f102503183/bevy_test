use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Id {
    id: u32,
}

#[derive(Component)]
pub struct HitPoint {
    hp: i32,
}
#[derive(Component)]
struct Movespeed {
    speed: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
struct GreetTimer(Timer);

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d); // Bundleとかくとエラーになる
    commands.spawn((
        Id { id: 0 },
        HitPoint { hp: 100 },
        Movespeed { speed: 1.0 },
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Player,
    ));
}

fn check_entety(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Id, &HitPoint)>) {
    if timer.0.tick(time.delta()).just_finished() {
        for (id, hp) in &query {
            println!("entety ID : {} health : {}", id.id, hp.hp);
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movespeed), With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, movespeed) in query.iter_mut() {
        if key_input.pressed(KeyCode::KeyD) {
            transform.translation.x += 100.0 * movespeed.speed * time.delta_secs();
        } else if key_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= 100.0 * movespeed.speed * time.delta_secs();
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .insert_resource(GreetTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .add_systems(Update, move_player)
            .add_systems(Update, check_entety);
    }
}
