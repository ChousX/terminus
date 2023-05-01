use super::Selector;
use crate::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

pub fn dysplay_selection(selector: Res<Selector>, mut lines: ResMut<DebugLines>) {
    if let Some((one, two)) = selector.get_space() {
        let three = Vec2::new(one.x, two.y);
        let four = Vec2::new(two.x, one.y);
        lines.line(one.extend(0.0), three.extend(0.0), 0.0);
        lines.line(one.extend(0.0), four.extend(0.0), 0.0);
        lines.line(two.extend(0.0), three.extend(0.0), 0.0);
        lines.line(two.extend(0.0), four.extend(0.0), 0.0);
    }
}
