pub mod binding;
use crate::prelude::*;

use self::binding::ModBindings;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        //All
        app.init_resource::<Input>().init_resource::<ModBindings>();
        //Keyboard
        app.init_resource::<binding::keyboard::KeyboardBindings>()
            .add_system(binding::keyboard::camera_move.run_if(is_keyboard));
        //Mouse&Keyboard
        app.init_resource::<binding::mouse_keyboard::MouseKeyboardBindings>()
            .add_systems(
                (
                    binding::mouse_keyboard::camera_move_keys,
                    binding::mouse_keyboard::camera_move_mouce,
                )
                    .distributive_run_if(is_mouce_and_keyboard),
            );
        info!("PluginLoaded");
    }
}

#[derive(Debug, Default, Resource, PartialEq, SystemSet, Clone, Eq, Hash)]
pub enum Input {
    #[default]
    MouceAndKeyboard,
    Keyboard,
    Controller,
    TouchScreen,
}

pub fn is_mouce_and_keyboard(input_state: Res<Input>) -> bool {
    Input::MouceAndKeyboard == *input_state
}

pub fn is_keyboard(input_state: Res<Input>) -> bool {
    Input::Keyboard == *input_state
}

pub fn is_touch_screen(input_state: Res<Input>) -> bool {
    Input::TouchScreen == *input_state
}

pub fn is_controller(input_state: Res<Input>) -> bool {
    Input::Controller == *input_state
}
