use crate::prelude::*;
use rand::prelude::*;

pub trait MapFeatureBuilder {
    type Out;
    fn build(&self) -> Self::Out;
}

#[derive(Component, Default)]
pub struct Water;

pub struct RiverBuilder {
    river_sorce_location: Option<Vec2>,
    max_angle_abs: Option<f32>,
    distance: Option<f32>,
    node_count: usize,
    bounding_box: (Vec2, Vec2),
    fork_chance: Option<f32>,
    max_forks: usize,
}
struct RiverNode {
    nodes: Vec<RiverNode>,
    pos: Vec2,
}

impl RiverNode {
    pub fn river_walk(&self, prev: Option<Vec2>) -> Vec<(Vec2, Vec2)> {
        let mut lines = if let Some(pos) = prev {
            self.nodes
                .iter()
                .map(|node| {
                    let this_pos = node.pos;
                    (pos, this_pos)
                })
                .collect()
        } else {
            Vec::default()
        };
        for node in &self.nodes {
            lines.append(&mut node.river_walk(Some(self.pos)))
        }
        lines
    }
}
