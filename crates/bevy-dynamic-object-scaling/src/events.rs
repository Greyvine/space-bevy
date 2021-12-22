use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct ScalingTranslationEvent {
    translation: Vec3,
}

impl ScalingTranslationEvent {
    pub fn new(other: &Vec3) -> Self {
        Self {
            translation: *other,
        }
    }
}

impl Deref for ScalingTranslationEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.translation
    }
}
