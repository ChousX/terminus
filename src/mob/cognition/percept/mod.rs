mod hearing;
mod smelling;
mod vision;
pub use self::{hearing::*, smelling::*, vision::*};
use crate::prelude::*;

pub enum PerceptionEvent {
    Seen(Entity),
    Sent(Entity),
}

impl PerceptionEvent {
    pub fn handle(mut events: EventReader<Self>) {}
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PerceptSet;

#[derive(Debug, Default, Bundle)]
pub struct PerceptionBundle {
    vision: Vision,
    seen: Visible,
    olfactory: Olfactory,
    smell: SmellAmmeter,
}
