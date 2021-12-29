use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct OrginRebasingEvent {
    pub translation: Vec3,
}

impl OrginRebasingEvent {
    pub fn new(other: &Vec3) -> Self {
        Self {
            translation: *other,
        }
    }
}

impl Deref for OrginRebasingEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.translation
    }
}
