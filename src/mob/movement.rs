use super::cognition::SoundEvent;
use super::Mob;
use crate::prelude::*;
use crate::selection::Selector;

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
        time: Res<Time>,
    ) {
        let speed = 5.0;
        let rotation_speed = 0.125; // adjust this value to change the rotation
        for event in events.iter() {
            use MobMoveEvent::*;
            match *event {
                MoveTowards(target, entity) => {
                    if let Ok(mut transform) = query.get_mut(entity) {
                        // Calculate the direction vector from the entity's position to the target position
                        let direction = target - transform.translation.truncate();
                        // Normalize the direction vector to get a unit vector
                        let direction_normalized = direction.normalize();
                        // Update the entity's position based on its speed and the direction vector
                        transform.translation +=
                            direction_normalized.extend(0.0) * speed * time.delta_seconds();
                    }
                }

                MoveAway(target, entity) => {
                    if let Ok(mut transform) = query.get_mut(entity) {
                        // Calculate the direction vector from the
                        let direction = transform.translation.truncate() - target;
                        // Normalize the direction vector to get a unit vector
                        let direction_normalized = direction.normalize();
                        // Update the entity's position based on its speed and the direction vector
                        transform.translation +=
                            direction_normalized.extend(0.0) * speed * time.delta_seconds();
                    }
                }

                FaceAway(target, entity) => {
                    if let Ok(mut transform) = query.get_mut(entity) {
                        // Calculate the direction vector from the target position to the entity's position
                        let direction = transform.translation.truncate() - target;
                        // Calculate the angle between the direction vector and the x-axis
                        let target_angle = direction.y.atan2(direction.x);
                        let current_angle = transform.rotation.to_axis_angle().1;
                        let diff_angle = (target_angle - current_angle
                            + std::f32::consts::FRAC_PI_2)
                            .rem_euclid(std::f32::consts::TAU)
                            - std::f32::consts::PI;
                        // Calculate the rotation speed
                        let max_rotation = rotation_speed * time.delta_seconds();
                        let actual_rotation = diff_angle.min(max_rotation).max(-max_rotation);
                        let mut new_angle =
                            (current_angle + actual_rotation) % (std::f32::consts::PI * 2.0);
                        // Update the entity's rotation to face away from the target over time
                        if new_angle < std::f32::EPSILON {
                            new_angle += (std::f32::consts::PI * 2.0);
                        }
                        transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
                    }
                }
                FaceTowards(target, entity) => {
                    if let Ok(mut transform) = query.get_mut(entity) {
                        // Calculate the direction vector from the entity's position to the target position
                        let direction = target - transform.translation.truncate();
                        // Calculate the angle between the direction vector and the x-axis
                        let target_angle = direction.y.atan2(direction.x);
                        let current_angle = transform.rotation.to_axis_angle().1;
                        let diff_angle = (target_angle - current_angle
                            + std::f32::consts::FRAC_PI_2)
                            .rem_euclid(std::f32::consts::TAU)
                            - std::f32::consts::PI;
                        // Calculate the rotation speed
                        let max_rotation = rotation_speed * time.delta_seconds();
                        let actual_rotation = diff_angle.min(max_rotation).max(-max_rotation);
                        let mut new_angle =
                            (current_angle + actual_rotation) % (std::f32::consts::PI * 2.0);
                        // Update the entity's rotation to face towards the target over time
                        if new_angle < std::f32::EPSILON {
                            new_angle += (std::f32::consts::PI * 2.0);
                        }
                        transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
                    }
                }
            }
        }
    }
}

pub fn test(
    selector: Res<Selector>,
    query: Query<Entity, With<Mob>>,
    mut movement: EventWriter<MobMoveEvent>,
) {
    //
    for mob in query.iter() {
        movement.send(MobMoveEvent::MoveTowards(selector.position, mob));
        movement.send(MobMoveEvent::FaceTowards(selector.position, mob));
    }
}
