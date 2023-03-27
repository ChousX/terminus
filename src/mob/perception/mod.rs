// Vistion
// Hearing
// Smell
// Touch
mod hearing;
mod smelling;
mod vision;

pub use self::{hearing::sound, smelling::sent, vision::sight};

use crate::prelude::*;

pub enum PerceptionEvent {}

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct PerceptSet;
