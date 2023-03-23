use crate::prelude::*;
pub enum CameraEvent {
    Zoom(f32),
    MoveTo(Vec3),
    MoveBy(Vec3),
    MoveDirectiion(Direction),
}

pub struct Velocity {
    pub data: Vec3,
    pub friction: f32,
}
impl Default for Velocity {
    fn default() -> Self {
        Self {
            data: Vec3::ZERO,
            friction: 0.5,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//TODO
pub fn camera_event_handler(
    mut events: EventReader<CameraEvent>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut velocity: Local<Velocity>,

    time: Res<Time>,
) {
    let (mut camera_pos, mut projection) = query
        .get_single_mut()
        .expect("Failed to get 1 and only 1 camera");
    let v = &mut velocity;

    for event in events.iter() {
        match event {
            CameraEvent::Zoom(amount) => {
                projection.scale += amount * time.delta_seconds();
                if projection.scale < 0.001 {
                    projection.scale = 0.0;
                }
            }
            CameraEvent::MoveTo(pos) => {}
            CameraEvent::MoveBy(amount) => {
                v.data += *amount;
            }
            CameraEvent::MoveDirectiion(dir) => match dir {
                Direction::Up => {}
                Direction::Down => {}
                Direction::Left => {}
                Direction::Right => {}
            },
        }
    }

    let z = v.data.z;
    let mut change = v.data * time.delta_seconds();
    change.z = 0.0;
    camera_pos.translation += change;
    camera_pos.translation.z = z;

    //friction
    let friction = v.friction * time.delta_seconds();

    if v.data.x.abs() < friction {
        v.data.x = 0.0;
    } else {
        if v.data.x >= 0.0 {
            v.data.x -= friction;
        } else {
            v.data.x += friction;
        }
    }

    if v.data.y.abs() < friction {
        v.data.y = 0.0;
    } else {
        if v.data.y >= 0.0 {
            v.data.y -= friction;
        } else {
            v.data.y += friction;
        }
    }
}
