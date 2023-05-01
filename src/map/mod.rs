mod features;

use crate::prelude::*;

//TODO: This hole thing needs to be built

//Marker for a Map Entity
#[derive(Component, Default)]
pub struct Map;

impl Chunkable for Map {}

pub type MapChunks = ChunkManager<Map>;

pub trait MapBuilder {}

#[derive(Bundle)]
pub struct MapBundle {
    map: Map,
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapChunks>()
            .add_system(MapChunks::update);
    }
}
