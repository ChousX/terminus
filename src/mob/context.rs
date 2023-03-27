use super::perception::PerceptionEvent;
use super::Mob;
use crate::prelude::*;

#[derive(Component)]
pub struct Context;

pub fn build_context(
    perception: EventReader<PerceptionEvent>,
    mobs: Query<(&mut Context, Entity), With<Mob>>,
) {
    todo!()
}
