use bevy_prototype_debug_lines::DebugLines;

use crate::{mob::MobChunks, prelude::*};

use super::PerceptionEvent;

#[derive(Component, Debug)]
pub struct Vision {
    range: f32,
    angle: f32,
}

impl Default for Vision {
    fn default() -> Self {
        Self {
            range: 150.0,
            angle: 40.0,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Visible;

pub fn sight(
    query: Query<(&Transform, &Vision, Entity)>,
    visible_things: Query<(&Transform), With<Visible>>,
    chunk: Res<MobChunks>,
    mut out: EventWriter<PerceptionEvent>,
    mut lines: ResMut<DebugLines>,
) {
    for (transform, vision, e) in query.iter() {
        let triangle = build_triangle(transform, vision.range, vision.angle);
        {
            lines.line(triangle[0].extend(0.0), triangle[1].extend(0.0), 0.0);
            lines.line(triangle[0].extend(0.0), triangle[2].extend(0.0), 0.0);
            lines.line(triangle[2].extend(0.0), triangle[1].extend(0.0), 0.0);
        }
        let chunk_range = {
            let mut x_min = triangle[2].x;
            let mut x_max = triangle[2].x;
            let mut y_min = triangle[2].y;
            let mut y_max = triangle[2].y;
            for i in 0..2 {
                let subjet = &triangle[i];
                if subjet.x < x_min {
                    x_min = subjet.x;
                } else if subjet.x > x_max {
                    x_max = subjet.x
                }
                if subjet.y < y_min {
                    y_min = subjet.y
                } else if subjet.y > y_max {
                    y_max = subjet.y
                }
            }
            let out = (Vec2::new(x_min, y_min), Vec2::new(x_max, y_max));
            out
        };
        let local_entities = chunk.get_in_range(chunk_range.0, chunk_range.1).into_iter();
        for thing_e in local_entities {
            if e == thing_e {
                continue;
            }
            if let Ok(thing) = visible_things.get(thing_e) {
                if in_triangle(&triangle, thing.translation.truncate()) {
                    out.send(PerceptionEvent::Seen(thing_e));
                }
            }
        }
    }
}

fn build_triangle(transform: &Transform, range: f32, angle: f32) -> [Vec2; 3] {
    let rotation = transform.rotation;
    let position = transform.translation.truncate();
    let vertex1 = position + (rotation * Vec3::new(0.0, 0.0, 0.0)).truncate();
    let vertex2 = position + (rotation * Vec3::new(angle, range, 0.0)).truncate();
    let vertex3 = position + (rotation * Vec3::new(-angle, range, 0.0)).truncate();
    [vertex1, vertex2, vertex3]
}

fn in_triangle(triangle: &[Vec2; 3], spot: Vec2) -> bool {
    (spot.x >= triangle[0].x && spot.x <= triangle[1].x)
        && (spot.y >= triangle[0].y && spot.y <= triangle[2].y)
}
