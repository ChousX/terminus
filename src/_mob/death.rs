use bevy::utils::HashSet;

use crate::prelude::*;

pub struct DeathEvent(pub Entity);

//the place to add on death things...
pub fn death_handler(mut commands: Commands, mut events: EventReader<DeathEvent>) {
    let mut kill_list = HashSet::new();
    for &DeathEvent(e) in events.iter() {
        kill_list.insert(e);
    }
    if !kill_list.is_empty() {
        for kill in kill_list.into_iter() {
            commands.entity(kill).despawn();
        }
    }
}
