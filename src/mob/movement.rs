use super::cognition::Vision;
use super::Mob;
use crate::prelude::*;
use crate::selection::Selector;

pub enum MobMoveEvent {
    MoveTowards(Vec2, Entity),
    _FaceTowards(Vec2, Entity),
}

impl MobMoveEvent {
    pub fn handle(
        mut events: EventReader<MobMoveEvent>,
        mut query: Query<(&mut Transform, &MobMoveData, &Vision)>,
        time: Res<Time>,
    ) {
        for event in events.iter() {
            use MobMoveEvent::*;
            match *event {
                MoveTowards(target, entity) => {
                    if let Ok((mut transform, move_data, vision)) = query.get_mut(entity) {
                        move_data.turn(target, &mut transform, time.delta_seconds());
                        //TODO:
                        let in_vistion_angle = true;
                        if in_vistion_angle {
                            move_data.move_to(target, &mut transform, time.delta_seconds());
                        }
                    }
                }
                _FaceTowards(target, entity) => {
                    if let Ok((mut transform, move_data, _)) = query.get_mut(entity) {
                        move_data.turn(target, &mut transform, time.delta_seconds());
                    }
                }
            }
        }
    }
}

#[derive(Component)]
pub struct MobMoveData {
    move_speed: f32,
    rotation_speed: f32,
}

impl Default for MobMoveData {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            rotation_speed: 3.0,
        }
    }
}

impl MobMoveData {
    pub fn turn(&self, target: Vec2, transform: &mut Mut<Transform>, time_delta: f32) {
        // Calculate the direction vector from the entity's position to the target position
        let direction = target - transform.translation.truncate();
        // Calculate the angle between the direction vector and the x-axis
        let target_angle = direction.y.atan2(direction.x);
        let current_angle = transform.rotation.to_axis_angle().1;
        let diff_angle = (target_angle - current_angle + std::f32::consts::FRAC_PI_2)
            .rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;
        // Calculate the rotation speed
        let max_rotation = self.rotation_speed * time_delta;
        let actual_rotation = diff_angle.min(max_rotation).max(-max_rotation);
        let mut new_angle = (current_angle + actual_rotation) % (std::f32::consts::PI * 2.0);
        // Update the entity's rotation to face towards the target over time
        if new_angle < std::f32::EPSILON {
            new_angle += (std::f32::consts::PI * 2.0);
        }
        transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
    }

    pub fn move_to(&self, target: Vec2, transform: &mut Mut<Transform>, time_delta: f32) {
        // Calculate the direction vector from the entity's position to the target position
        let direction = target - transform.translation.truncate();
        // Normalize the direction vector to get a unit vector
        let direction_normalized = direction.normalize();
        // Update the entity's position based on its speed and the direction vector
        transform.translation += direction_normalized.extend(0.0) * self.move_speed * time_delta;
        //
    }

    pub fn _move_away(&self, target: Vec2, transform: &mut Mut<Transform>, time_delta: f32) {
        // Calculate the direction vector from the
        let direction = transform.translation.truncate() - target;
        // Normalize the direction vector to get a unit vector
        let direction_normalized = direction.normalize();
        // Update the entity's position based on its speed and the direction vector
        transform.translation += direction_normalized.extend(0.0) * self.move_speed * time_delta;
    }

    pub fn _turn_away(&self, target: Vec2, transform: &mut Mut<Transform>, time_delta: f32) {
        // Calculate the direction vector from the target position to the entity's position
        let direction = transform.translation.truncate() - target;
        // Calculate the angle between the direction vector and the x-axis
        let target_angle = direction.y.atan2(direction.x);
        let current_angle = transform.rotation.to_axis_angle().1;
        let diff_angle = (target_angle - current_angle + std::f32::consts::FRAC_PI_2)
            .rem_euclid(std::f32::consts::TAU)
            - std::f32::consts::PI;
        // Calculate the rotation speed
        let max_rotation = self.rotation_speed * time_delta;
        let actual_rotation = diff_angle.min(max_rotation).max(-max_rotation);
        let mut new_angle = (current_angle + actual_rotation) % (std::f32::consts::PI * 2.0);
        // Update the entity's rotation to face away from the target over time
        if new_angle < std::f32::EPSILON {
            new_angle += (std::f32::consts::PI * 2.0);
        }
        transform.rotation = Quat::from_axis_angle(Vec3::Z, new_angle);
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
    }
}
