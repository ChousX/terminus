use super::{
    input::{MouseSensitivity, MouseZoomSensitivity},
    *,
};
use crate::prelude::*;

pub struct CameraDebugPlugin;
impl Plugin for CameraDebugPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MouseSensitivity>()
            .register_type::<MouseZoomSensitivity>();
    }
}
