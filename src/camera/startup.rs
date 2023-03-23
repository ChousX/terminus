use crate::prelude::*;

pub fn new_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
