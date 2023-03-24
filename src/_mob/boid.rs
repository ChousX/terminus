use super::Vision;
use crate::prelude::*;

use super::{SpeedEvent, SteerEvent};

pub fn steer_boid_tords_boids(
    query: Query<(&Transform, &Vision, Entity)>,
    mut output: EventWriter<SteerEvent>,
) {
    let boids: Vec<(&Transform, &Vision, Entity)> = query.iter().collect();
    for i in 0..boids.len() {
        let (trans, vision, id) = boids[i];
        for j in (i + 1)..boids.len() {
            let (other, _, _) = boids[j];
            if vision.can_see(trans, other) {
                output.send(SteerEvent::Tords(other.translation.truncate(), id))
            }
        }
    }
}

pub fn steer_boid_awayfrom_boids(
    query: Query<(&Transform, &Vision, Entity)>,
    mut output: EventWriter<SteerEvent>,
) {
    let boids: Vec<(&Transform, &Vision, Entity)> = query.iter().collect();
    for i in 0..boids.len() {
        let (trans, vision, id) = boids[i];
        for j in (i + 1)..boids.len() {
            let (other, _, _) = boids[j];
            if vision.is_touching(19.0, trans, other) {
                info!(true);
                output.send(SteerEvent::Away(other.translation.truncate(), id))
            }
        }
    }
}

pub fn avrige_boid_speed_dir(
    query: Query<(&Transform, &Vision, Entity)>,
    mut steer: EventWriter<SteerEvent>,
    mut speed: EventWriter<SpeedEvent>,
) {
    let boids: Vec<(&Transform, &Vision, Entity)> = query.iter().collect();
    for i in 0..boids.len() {
        let (trans, vision, _id) = boids[i];
        for j in (i + 1)..boids.len() {
            let (other, _, _) = boids[j];
            todo!()
        }
    }
}
