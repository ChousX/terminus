use bevy::{asset::update_asset_storage_system, ecs::entity::ReserveEntitiesIterator};

use super::{Context, Mob};
use crate::prelude::*;

pub struct MobBundle {
    mob: Mob,
    context: Context,
}

pub enum MobMoveEvent {
    FaceTowards(Vec2, Entity),
    FaceAway(Vec2, Entity),
    MoveAway(Vec2, Entity),
    MoveTowards(Vec2, Entity),
}

impl MobMoveEvent {
    pub fn handle(
        mut events: EventReader<MobMoveEvent>,
        mut query: Query<&mut Transform, With<Mob>>,
    ) {
        for event in events.iter() {
            use MobMoveEvent::*;
            match *event {
                MoveTowards(target, entity) => if let Ok(mut transform) = query.get_mut(entity) {},
                MoveAway(target, entity) => if let Ok(mut transform) = query.get_mut(entity) {},
                FaceAway(target, entity) => if let Ok(mut transform) = query.get_mut(entity) {},
                FaceTowards(target, entity) => if let Ok(mut transform) = query.get_mut(entity) {},
            }
        }
    }
}
