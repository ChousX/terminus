mod pathing;
use crate::{prelude::*, select::Selector};
use std::ops::Neg;

pub use pathing::{add_move_path_command, Pathing};

use super::Mob;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementEvent>()
            .add_system(movement_event_handler.run_if(on_event::<MovementEvent>()))
            .add_system(pathing::path_to)
            .add_system(pathing::view_path);
    }
}

pub enum MovementEvent {
    MoveToSpot(Vec2, Entity),
    MoveToThing(Entity, Entity),
    MoveAwayFrom(Vec2, Entity),
    MoveAwayFromThing(Entity, Entity),
    Rotate(f32, Entity),
    RotateToSpot(Vec2, Entity),
}

pub fn movement_event_handler(
    mut events: EventReader<MovementEvent>,
    mut query: Query<(&mut Transform, &Mob)>,
    time: Res<Time>,
) {
    use MovementEvent::*;
    for event in events.iter() {
        match *event {
            MoveToSpot(target, e) => {
                if let Ok((mut transform, mob)) = query.get_mut(e) {
                    rotate(&mut transform, target, mob, time.delta_seconds());
                    move_to(&mut transform, mob, target, time.delta_seconds());
                }
            }

            MoveToThing(to_e, e) => {
                let target = if let Ok(target) = query.get_component::<Transform>(to_e) {
                    target.translation.truncate()
                } else {
                    continue;
                };
                if let Ok((mut transform, mob)) = query.get_mut(e) {
                    rotate(&mut transform, target, mob, time.delta_seconds());
                    move_to(&mut transform, mob, target, time.delta_seconds());
                }
            }

            Rotate(amount, e) => {
                if let Ok((mut transform, mob)) = query.get_mut(e) {
                    if amount.abs() > mob.turn_rate {
                        transform.rotate_z(mob.turn_rate * time.delta_seconds());
                    }
                    transform.rotate_z(amount * time.delta_seconds());
                }
            }

            RotateToSpot(spot, e) => {
                if let Ok((mut transform, mob)) = query.get_mut(e) {
                    rotate(&mut transform, spot, mob, time.delta_seconds());
                }
            }
            _ => {}
        }
    }
}

pub fn move_to(transform: &mut Mut<Transform>, mob: &Mob, target: Vec2, sec: f32) {
    if !is_facing(&transform, 0.002, target) {
        return;
    }
    let forward = forward_normal(transform.rotation);
    transform.translation += (forward * Vec2::splat(mob.speed) * sec).extend(0.0);
}

pub fn rotate(transform: &mut Mut<Transform>, target: Vec2, mob: &Mob, delta: f32) {
    let pos = transform.translation.truncate();
    let pos_headding = forward_normal(transform.rotation);
    let target_headding = to_normal(target, pos);
    let dot = pos_headding.dot(target_headding);

    if (dot - 1.).abs() < f32::EPSILON {
        return;
    }

    let right = (transform.rotation * Vec3::X).truncate();
    let right_dot_target = right.dot(target_headding);
    let rotation_sign = -f32::copysign(1.0, right_dot_target);

    let max_angle = dot.clamp(-1.0, 1.0).acos(); // clamp acos for safety
    let rotation_angle = rotation_sign * (mob.turn_rate * delta).min(max_angle);
    transform.rotate_z(rotation_angle);
}

pub fn is_facing(transform: &Mut<Transform>, factor: f32, target: Vec2) -> bool {
    let pos = transform.translation.truncate();
    let pos_headding = forward_normal(transform.rotation);
    let target_headding = to_normal(target, pos);
    let angle = pos_headding.dot(target_headding).neg();

    angle < factor
}

pub fn forward_normal(rotation: Quat) -> Vec2 {
    (rotation * Vec3::Y).truncate()
}

pub fn to_normal(target: Vec2, translation: Vec2) -> Vec2 {
    (target - translation).normalize()
}

pub fn rotation_test(
    selector: Res<Selector>,
    query: Query<Entity, With<Mob>>,
    mut out: EventWriter<MovementEvent>,
) {
    for e in query.iter() {
        out.send(MovementEvent::MoveToSpot(selector.pos, e))
    }
}
