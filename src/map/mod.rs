mod features;
use bevy::utils::HashMap;

use crate::prelude::*;

//TODO: This hole thing needs to be built

//Marker for a Map Entity
<<<<<<< HEAD
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
=======
pub struct Map{}

impl Chunkable for Map {}

pub trait MapBuilder {}

pub struct MapBundle{
    map: Map
}

pub struct MapPlugin{}
>>>>>>> 7e102f4 (added some todo's)
