use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct ForceEvent {
    force: Vec3,
}

impl ForceEvent {
    pub fn new(other: &Vec3) -> Self {
        Self { force: *other }
    }
}

impl Deref for ForceEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.force
    }
}
