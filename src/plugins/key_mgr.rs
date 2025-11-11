use bevy::prelude::*;

pub struct KeyMgr;

fn keybord_input_system(keybord_input: Res<ButtonInput<KeyCode>>) {
    if keybord_input.pressed(KeyCode::Space) {
        println!("space is pressed!");
    }
}

impl Plugin for KeyMgr {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keybord_input_system);
    }
}
