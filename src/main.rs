use bevy::prelude::*;

mod plugins {
    pub mod key_mgr;
    pub mod player_plugin;
    pub mod world_setup;
}
use plugins::key_mgr::KeyMgr;
use plugins::player_plugin::PlayerPlugin;
use plugins::world_setup::WorldSetup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldSetup)
        .add_plugins(KeyMgr)
        .run();
}
