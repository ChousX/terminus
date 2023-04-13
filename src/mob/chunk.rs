use super::Mob;
use bevy::utils::{HashMap, HashSet};

use crate::prelude::*;

#[derive(Resource)]
pub struct MobChunks {
    chunks: HashMap<IVec2, Chunk>,
    chunk_size: Vec2,
}

impl MobChunks {
    fn add(&mut self, pos: Vec2, entity: Entity) {
        let chunk_pos = self.whitch_chunk(pos);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.insert(entity);
        } else {
            let mut set = HashSet::new();
            set.insert(entity);
            self.chunks.insert(chunk_pos, Chunk(set));
        }
    }

    pub fn get_in_range(&self, one: Vec2, two: Vec2) -> Vec<Entity> {
        let (min, max) = {
            let (x_min, x_max) = if one.x > two.x {
                (two.x, one.x)
            } else {
                (one.x, two.x)
            };
            let (y_min, y_max) = if one.y > two.y {
                (two.y, one.y)
            } else {
                (one.y, two.y)
            };
            (Vec2::new(x_min, y_min), Vec2::new(x_max, y_max))
        };
        let start_chunk = {
            let pos = (min / self.chunk_size).floor();
            IVec2::new(pos.x as i32, pos.y as i32)
        };
        let delta = ((max - min) / self.chunk_size).floor();
        let mut entities: Vec<Entity> = Vec::new();
        for x in 0..(delta.x as i32) {
            for y in 0..(delta.y as i32) {
                let pos = IVec2::new(x, y) + start_chunk;
                if let Some(chunk) = self.chunks.get(&pos) {
                    entities.extend(chunk.iter());
                    //chunk.iter().collect()
                }
            }
        }
        entities
    }

    pub fn get(&self, pos: Vec2) -> Option<&Chunk> {
        let chunk_pos = self.whitch_chunk(pos);
        self.chunks.get(&chunk_pos)
    }

    fn whitch_chunk(&self, pos: Vec2) -> IVec2 {
        let size = self.chunk_size;
        let out = (pos / size).floor();
        IVec2::new(out.x as i32, out.y as i32)
    }

    pub fn update(mut chunks: ResMut<Self>, query: Query<(&Transform, Entity), With<Mob>>) {
        let mut new_chunks = Self::default();
        for (transform, entity) in query.iter() {
            new_chunks.add(transform.translation.truncate(), entity);
        }
        *chunks = new_chunks;
    }
}

impl Default for MobChunks {
    fn default() -> Self {
        Self {
            chunk_size: Vec2::new(100.0, 100.0),
            chunks: default(),
        }
    }
}

#[derive(Deref, DerefMut, Debug, Default)]
pub struct Chunk(pub HashSet<Entity>);

pub mod debug {
    use super::*;
    use bevy::prelude::*;
    use bevy_prototype_debug_lines::DebugLines;

    pub fn dysplay_boxes(mut lines: ResMut<DebugLines>, chunks: Res<MobChunks>) {
        let chunk = chunks.chunks.keys();
        for chunk in chunk {
            let p1 = (Vec2::new(chunk.x as f32, chunk.y as f32) * chunks.chunk_size).extend(0.0);
            let p3 = p1 + chunks.chunk_size.extend(0.0);
            let p2 = Vec3::new(p3.x, p1.y, 0.0);
            let p4 = Vec3::new(p1.x, p3.y, 0.0);
            lines.line(p1, p2, 0.0);
            lines.line(p2, p3, 0.0);
            lines.line(p3, p4, 0.0);
            lines.line(p4, p1, 0.0);
        }
    }
}
