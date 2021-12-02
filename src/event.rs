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

#[derive(Debug, Default)]
pub struct PitchEvent {
    pitch: f32,
}

impl PitchEvent {
    pub fn new(value: f32) -> Self {
        Self { pitch: value }
    }
}

impl Deref for PitchEvent {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.pitch
    }
}

#[derive(Debug, Default)]
pub struct YawEvent {
    yaw: f32,
}

impl YawEvent {
    pub fn new(value: f32) -> Self {
        Self { yaw: value }
    }
}

impl Deref for YawEvent {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.yaw
    }
}
