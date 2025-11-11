use bevy::prelude::*;

mod plugins {
    pub mod key_mgr;
    pub mod player_plugin;
}
use plugins::key_mgr::KeyMgr;
use plugins::player_plugin::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(KeyMgr)
        .run();
}
