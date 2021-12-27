use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct BillboardingTranslationEvent {
    pub translation: Vec3,
}

impl BillboardingTranslationEvent {
    pub fn new(other: &Vec3) -> Self {
        Self {
            translation: *other,
        }
    }
}

impl Deref for BillboardingTranslationEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.translation
    }
}
