use bevy::app::Plugin;

use {character_controller::CharacterControllerPlugin, input_mode::InputModePlugin};

pub mod character_controller;
pub mod input_mode;
pub mod solid_body_motion;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((CharacterControllerPlugin, InputModePlugin));
    }
}
