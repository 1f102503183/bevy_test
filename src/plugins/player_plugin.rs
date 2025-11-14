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
    asset_server: Res<AssetServer>, // mut meshes: ResMut<Assets<Mesh>>,
                                    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d); // Bundleとかくとエラーになる
    commands.spawn(PointLight {
        color: Color::srgb(1.0, 1.0, 1.0),
        intensity: 10.0,
        range: 100.0,
        radius: 3.0,
        ..default()
    });
    commands.spawn((
        Id { id: 0 },
        HitPoint { hp: 100 },
        Movespeed { speed: 1.0 },
        Sprite {
            image: asset_server.load("sprite/player/tmp.png"),
            custom_size: Some(Vec2::new(150.0, 150.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        // MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
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
        let mut direction = Vec3::ZERO;
        if key_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        } else if key_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if direction != Vec3::ZERO {
            transform.translation +=
                direction.normalize() * movespeed.speed * time.delta_secs() * 100.0;
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
