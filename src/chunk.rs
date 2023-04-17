use std::marker::PhantomData;

use bevy::utils::{HashMap, HashSet};

use crate::prelude::*;

#[derive(Resource)]
pub struct ChunkManager<T: Chunkable> {
    chunks: HashMap<IVec2, HashSet<Entity>>,
    chunk_size: Vec2,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Chunkable> ChunkManager<T> {
    fn add(&mut self, pos: Vec2, entity: Entity) {
        let chunk_pos = self.whitch_chunk(pos);
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            chunk.insert(entity);
        } else {
            let mut set = HashSet::new();
            set.insert(entity);
            self.chunks.insert(chunk_pos, set);
        }
    }

    fn whitch_chunk(&self, pos: Vec2) -> IVec2 {
        let size = self.chunk_size;
        let out = (pos / size).floor();
        IVec2::new(out.x as i32, out.y as i32)
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

    pub fn get(&self, pos: Vec2) -> Option<&HashSet<Entity>> {
        let chunk_pos = self.whitch_chunk(pos);
        self.chunks.get(&chunk_pos)
    }

    pub fn update(
        mut chunks: ResMut<ChunkManager<T>>,
        query: Query<(&Transform, Entity), With<T>>,
    ) {
        let mut new_chunks = Self::default();
        for (transform, entity) in query.iter() {
            new_chunks.add(transform.translation.truncate(), entity);
        }
        *chunks = new_chunks;
    }
}

impl<T: Chunkable> Default for ChunkManager<T> {
    fn default() -> Self {
        Self {
            chunks: HashMap::default(),
            chunk_size: Vec2::new(100., 100.),
            phantom: PhantomData,
        }
    }
}

pub trait Chunkable: Component + Default {}
