use std::collections::VecDeque;

use bevy_prototype_debug_lines::DebugLines;

use crate::{
    mob::Mob,
    prelude::*,
    select::{Selected, Selector},
};

use super::MovementEvent;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Pathing(VecDeque<Vec2>);
impl From<Vec2> for Pathing {
    fn from(value: Vec2) -> Self {
        let mut vec = VecDeque::new();
        vec.push_back(value);
        Self(vec)
    }
}
pub fn path_to(
    mut commands: Commands,
    mut query: Query<(&mut Pathing, &Transform, Entity)>,
    mut out: EventWriter<MovementEvent>,
) {
    for (mut pathing, transform, e) in query.iter_mut() {
        if let Some(pos) = pathing.front() {
            if transform.translation.truncate() == *pos {
                pathing.pop_front();
            } else {
                out.send(MovementEvent::MoveToSpot(*pos, e));
            }
        } else {
            commands.entity(e).remove::<Pathing>();
        }
    }
}

pub fn view_path(mut query: Query<(&Pathing, &Transform, Entity)>, mut lines: ResMut<DebugLines>) {
    for (path, transorm, e) in query.iter() {
        let mut last = transorm.translation.truncate();
        for point in path.iter() {
            lines.line(last.extend(0.0), point.extend(0.0), 0.0);
            let last = point;
        }
    }
}

///this needs run critera
pub fn add_move_path_command(
    mut commands: Commands,
    selector: Res<Selector>,
    selected: Res<Selected>,
    mut query: Query<Option<&mut Pathing>, With<Mob>>,
) {
    for e in selected.0.iter() {
        if let Ok(mut pathing) = query.get_mut(*e) {
            let new = selector.pos;
            if let Some(mut pathing) = pathing {
                pathing.push_back(new);
            } else {
                let pathing: Pathing = new.into();
                commands.entity(*e).insert(pathing);
            }
        }
    }
}
