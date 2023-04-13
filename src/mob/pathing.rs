use std::collections::LinkedList;

use super::{movement::MobMoveEvent, Mob};

use crate::{
    prelude::*,
    selection::{Selected, Selector},
};

#[derive(Component, Clone)]
pub struct MovePath(LinkedList<Vec2>);

impl MovePath {
    pub fn add_path(entity: Entity, commands: &mut Commands, path: Self) {
        commands.entity(entity).insert(path);
    }

    pub fn pop(&mut self) {
        self.0.pop_front();
    }

    pub fn remove_path(entity: Entity, commands: &mut Commands) {
        commands.entity(entity).remove::<Self>();
    }

    pub fn append(&mut self, mut path: MovePath) {
        self.0.append(&mut path.0);
    }

    pub fn new(point: Vec2) -> Self {
        let mut ll = LinkedList::default();
        ll.push_back(point);
        Self(ll)
    }
}

pub fn path_consumer(
    mut query: Query<(&mut MovePath, &Transform, Entity)>,
    mut mover: EventWriter<MobMoveEvent>,
    mut commands: Commands,
) {
    for (mut path, transform, e) in query.iter_mut() {
        if let Some(front) = path.0.front() {
            let current = transform.translation.truncate();
            let delta = (current - *front).abs();
            if delta.x <= f32::EPSILON && delta.y <= f32::EPSILON {
                path.pop();
            } else {
                mover.send(MobMoveEvent::MoveTowards(*front, e))
            }
        } else {
            MovePath::remove_path(e, &mut commands)
        }
    }
}

pub fn new_path_for_selected_mobs(
    selected: Res<Selected>,
    selector: Res<Selector>,
    mut commands: Commands,
    mut query: Query<Option<&mut MovePath>, With<Mob>>,
) {
    let new_path = MovePath::new(selector.position);
    for e in selected.iter() {
        if let Ok(Some(mut path)) = query.get_mut(*e) {
            *path = new_path.clone();
        } else {
            MovePath::add_path(*e, &mut commands, new_path.clone());
        }
    }
}

pub fn add_pos_to_selected_mobs(
    selected: Res<Selected>,
    selector: Res<Selector>,
    mut commands: Commands,
    mut query: Query<Option<&mut MovePath>, With<Mob>>,
) {
    let new_path = MovePath::new(selector.position);
    for e in selected.iter() {
        if let Ok(Some(mut path)) = query.get_mut(*e) {
            path.append(new_path.clone());
        } else {
            MovePath::add_path(*e, &mut commands, new_path.clone());
        }
    }
}
