use bevy::utils::HashMap;

use crate::prelude::*;

//TODO: This hole thing needs to be built

//Marker for a Map Entity
pub struct Map{}

impl Chunkable for Map {}

pub trait MapBuilder {}

pub struct MapBundle{
    map: Map
}

pub struct MapPlugin{}