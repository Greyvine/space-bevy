use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct ScalingCameraEvent {
    force: Vec3,
}

impl ScalingCameraEvent {
    pub fn new(other: &Vec3) -> Self {
        Self { force: *other }
    }
}

impl Deref for ScalingCameraEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.force
    }
}
