use super::{Cursor, Selectable, Selected, Selector, SelectorMovementEvent};
use crate::prelude::*;

pub fn selector_mouce_syncer(
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut out: EventWriter<SelectorMovementEvent>,
    window: Query<&Window>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = window.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        out.send(SelectorMovementEvent::MoveTo(world_position))
    }
}

pub fn cursor_selector_linker(
    selector: Res<Selector>,
    mut query: Query<&mut Transform, With<Cursor>>,
) {
    if let Ok(mut transform) = query.get_mut(selector.cursor) {
        transform.translation = selector.position.extend(0.0);
    }
}

pub fn start_selection(mut selector: ResMut<Selector>) {
    //info!("start");
    if selector.marker.is_none() {
        selector.mark()
    }
}

pub fn stop_selection(
    query: Query<(&Transform, Entity), With<Selectable>>,
    mut selector: ResMut<Selector>,
    mut selected: ResMut<Selected>,
) {
    //info!("stop");
    let mut selection = Vec::new();
    if let Some((bottem_left, top_right)) = selector.get_space() {
        //info!("{:?}|{:?}", bottem_left, top_right);
        for (transform, e) in query.iter() {
            let position = transform.translation.truncate();
            if in_space(bottem_left, top_right, position) {
                selection.push(e);
            }
        }
        selector.marker = None;
    }
    //info!("{:?}", &selection);
    selected.0 = selection;
}

#[inline]
fn in_space(s: Vec2, b: Vec2, subject: Vec2) -> bool {
    subject.x > s.x && subject.x <= b.x && subject.y > s.y && subject.y <= b.y
}
