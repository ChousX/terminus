pub mod binding;
use crate::mob::{add_pos_to_selected_mobs, new_path_for_selected_mobs};
use crate::prelude::*;
use crate::selection::{movement::SelectorMovementEvent, start_selection, stop_selection};

use self::binding::{Binding, ModBindings};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        //All
        app.init_resource::<Input>()
            .init_resource::<ModBindings>()
            .init_resource::<GeneralInputBindings>()
            .add_system(switch_input.run_if(should_switch_input));
        //Keyboard
        app.init_resource::<binding::keyboard::KeyboardBindings>()
            .add_systems(
                (
                    binding::keyboard::camera_move,
                    start_selection.run_if(binding::keyboard::selector_start),
                    stop_selection.run_if(binding::keyboard::selector_end),
                    binding::keyboard::move_selector.before(SelectorMovementEvent::handle),
                )
                    .distributive_run_if(is_keyboard),
            );
        //Mouse&Keyboard
        app.init_resource::<binding::mouse_keyboard::MouseKeyboardBindings>()
            .add_systems(
                (
                    binding::mouse_keyboard::camera_move_keys,
                    binding::mouse_keyboard::camera_move_mouce,
                    start_selection.run_if(binding::mouse_keyboard::selector_start),
                    stop_selection.run_if(binding::mouse_keyboard::selector_end),
                    add_pos_to_selected_mobs.run_if(binding::mouse_keyboard::append_path),
                    new_path_for_selected_mobs.run_if(binding::mouse_keyboard::new_path),
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
    _Controller,
    _TouchScreen,
}
impl Input {
    pub fn switch(&mut self) {
        let new = match *self {
            Input::Keyboard => Input::MouceAndKeyboard,
            Input::MouceAndKeyboard => Input::Keyboard,
            _ => Input::default(),
        };

        *self = new;
    }
}

pub fn is_mouce_and_keyboard(input_state: Res<Input>) -> bool {
    Input::MouceAndKeyboard == *input_state
}

pub fn is_keyboard(input_state: Res<Input>) -> bool {
    Input::Keyboard == *input_state
}

pub fn is_touch_screen(input_state: Res<Input>) -> bool {
    Input::_TouchScreen == *input_state
}

pub fn is_controller(input_state: Res<Input>) -> bool {
    Input::_Controller == *input_state
}

#[derive(Debug, Resource)]
pub struct GeneralInputBindings {
    switch_input: Binding,
}

impl Default for GeneralInputBindings {
    fn default() -> Self {
        Self {
            switch_input: Binding {
                keys: vec![Key::Board(KeyCode::Delete)],
                mask: None,
            },
        }
    }
}

pub fn should_switch_input(
    keys: Res<bevy::prelude::Input<KeyCode>>,
    mouse: Res<bevy::prelude::Input<MouseButton>>,
    modkes: Res<binding::ModBindings>,
    bindings: Res<GeneralInputBindings>,
) -> bool {
    modkes.check(&keys, &mouse, &bindings.switch_input)
}

pub fn switch_input(mut input: ResMut<Input>) {
    input.switch();
}
