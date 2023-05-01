use super::{actions::Action, context::Context};
use crate::prelude::*;
use std::collections::BinaryHeap;

#[derive(Default, DerefMut, Deref)]
pub struct Desires(BinaryHeap<Desire>);

pub struct Desire(i16, Action);

pub fn inclination(contexts: Query<(&Context, Entity)>) {
    todo!()
}

// Some traits to get ord working for the heap
impl PartialEq for Desire {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Desire {}

impl PartialOrd for Desire {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}
impl Ord for Desire {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
