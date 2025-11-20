use bevy::prelude::*;

// Main Player component
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

// 謎の挨拶タイマー
#[derive(Resource)]
struct GreetTimer(Timer);

// animation resorce
#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Resource, Default)]
struct MoveAnimation(Handle<LoadedFolder>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}
