use super::*;
use crate::prelude::*;

pub struct SelectDebugPlugin;
impl Plugin for SelectDebugPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selector>().register_type::<Selected>();
    }
}
