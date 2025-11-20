use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .insert_resource(GreetTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .add_systems(OnEnter(AppState::Setup), load_textures) // OnEnterでAppStateがSetupの時動くようになる
            // 2. AppState::Setup の間、毎フレーム読み込み完了をチェック
            .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
            .add_systems(OnEnter(AppState::Finished), setup_player)
            .add_systems(
                Update,
                (move_player, check_entety, animation).run_if(in_state(AppState::Finished)),
            );
    }
}
