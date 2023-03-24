use super::*;
use crate::prelude::*;

use crate::utils::check;
// TODO: add a run if so this is not running uless it need
pub fn in_univer_cursor_update(
    window: Query<&Window>,
    mut selector: ResMut<Selector>,
    query: Query<&Transform, With<Camera>>,
) {
    let window = window.single();
    if let Some(cursor) = window.cursor_position() {
        let camera_pos = query
            .get_single()
            .expect("There is not 1 camera...")
            .translation
            .truncate();
        selector.pos = cursor - (Vec2::new(window.width(), window.height()) * 0.5) + camera_pos;
    }
}

pub fn select(
    settings: Res<SelectorBindings>,
    mut selector: ResMut<Selector>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut output: EventWriter<SelectionEvent>,
) {
    if !check(&keys, &settings.start_mark, Some(&buttons)) {
        if let Some(mark) = selector.marker {
            // So this is the flow spot for a valid selection
            output.send(SelectionEvent::Rectangle {
                p1: selector.pos,
                p2: mark,
            });
            selector.marker = None;
        }
        return;
    }
    selector.marker = Some(selector.pos);
}

pub fn selection_handler(
    mut input: EventReader<SelectionEvent>,
    query: Query<(&Transform, Entity), With<Selectable>>,
    mut selected: ResMut<Selected>,
) {
    for selection in input.iter() {
        match selection {
            SelectionEvent::Rectangle { p1, p2 } => {
                let mut new_selection = Vec::new();
                //p1 bot left of the p2 top right
                let (p1, p2) = {
                    let (x1, x2) = if p1.x > p2.x {
                        (p1.x, p2.x)
                    } else {
                        (p2.x, p1.x)
                    };
                    let (y1, y2) = if p1.y > p2.y {
                        (p1.y, p2.y)
                    } else {
                        (p2.y, p1.y)
                    };
                    (Vec2::new(x2, y2), Vec2::new(x1, y1))
                };

                for (transform, entity) in query.iter() {
                    let Vec2 { x, y } = transform.translation.truncate();
                    //if its inside the selection area
                    if x >= p1.x && x <= p2.x && y >= p1.y && y <= p2.y {
                        new_selection.push(entity);
                    }
                }
                selected.0 = new_selection;
                info!("{:?}", selected.0);
            }
            SelectionEvent::Single(e) => {
                selected.0 = vec![*e];
            }
        }
    }
}
