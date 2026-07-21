use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

pub struct InputModePlugin;

impl Plugin for InputModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_input_mode);
    }
}

#[derive(Resource)]
pub enum InputMode {
    Game,
    Menu,
}

fn update_input_mode(
    input_mode: Res<InputMode>,
    mut q_cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if input_mode.is_changed() {
        let mut cursor = q_cursor.single_mut().unwrap();
        match *input_mode {
            InputMode::Game => {
                cursor.visible = false;
                cursor.grab_mode = CursorGrabMode::Locked;
                time.unpause();
            }
            InputMode::Menu => {
                cursor.visible = true;
                cursor.grab_mode = CursorGrabMode::None;
                time.pause();
            }
        }
    }
}
