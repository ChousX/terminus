use crate::prelude::*;

use super::Cursor;

#[derive(Resource)]
pub struct Selector {
    pub position: Vec2,
    pub marker: Option<Vec2>,
    pub cursor: Entity,
}

impl FromWorld for Selector {
    fn from_world(world: &mut World) -> Self {
        let (position, _e) = {
            let mut query = world.query::<(&Window, Entity)>();
            let (window, e) = query.single(world);
            let window_pos = Vec2::new(window.width(), window.height());
            (window_pos - (window_pos * Vec2::splat(0.5)), e)
        };
        let assets = world
            .get_resource::<AssetServer>()
            .expect("AssetServer Not Running");
        let cursor = world
            .spawn(SpriteBundle {
                texture: assets.load(r"test.png"),
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            })
            .insert(Cursor)
            .id();
        Self {
            position,
            marker: None,
            cursor,
        }
    }
}

impl Selector {
    pub fn get_space(&self) -> Option<(Vec2, Vec2)> {
        if let Some(marker) = self.marker.clone() {
            let pos = self.position;
            let x = if pos.x < marker.x {
                (pos.x, marker.x)
            } else {
                (marker.x, pos.x)
            };
            let y = if pos.y < marker.y {
                (pos.y, marker.y)
            } else {
                (marker.y, pos.y)
            };
            Some((Vec2::new(x.0, y.0), Vec2::new(x.1, y.1)))
        } else {
            None
        }
    }

    pub fn mark(&mut self) {
        self.marker = Some(self.position);
    }
}
