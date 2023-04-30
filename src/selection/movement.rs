use super::Selector;
use crate::prelude::*;
pub enum SelectorMovementEvent {
    MoveTowrds(Vec2),
    MoveTo(Vec2),
    Up,
    Down,
    Left,
    Right,
}

impl SelectorMovementEvent {
    //Asumming:
    // Camera Transform is uptodate
    pub fn handle(
        mut events: EventReader<SelectorMovementEvent>,
        mut selector: ResMut<Selector>,
        camera: Query<&Transform, With<Camera2d>>,
    ) {
        let mut last = None;
        let mut amount = Vec2::ZERO;
        for event in events.iter() {
            use SelectorMovementEvent::*;
            match *event {
                MoveTowrds(_spot) => {} //amount += spot,
                MoveTo(spot) => last = Some(spot),
                Up => {}
                Down => {}
                Left => {}
                Right => {}
            }
        }
        // if there is nothing to do end erly
        if !(last.is_some() || amount != Vec2::ZERO) {
            return;
        }
        if let Ok(camera_transform) = camera.get_single() {
            let camera_pos = camera_transform.translation.truncate();
            if let Some(pos) = last {
                selector.position = pos + camera_pos;
            }
            if amount != Vec2::ZERO {
                let dif = selector.position - camera_pos;
                selector.position = camera_pos + amount + dif;
            }
        }
    }
}
