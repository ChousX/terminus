mod error;
mod keybindings;

use bevy::prelude::KeyCode;
pub use error::{Error, Result};
pub use keybindings::{check, Bindable, Binding, Key, KeyBindings, ModKeyBindings, ModPressed};
