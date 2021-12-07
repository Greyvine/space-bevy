use bevy::prelude::*;

struct Mass(f32);

#[derive(Default)]
struct Velocity(Vec3);

#[derive(Default)]
struct Acceleration(Vec3);

#[derive(Bundle)]
pub struct BodyBundle {
    mass: Mass,
    transform: Transform,
    vel: Velocity,
    acc: Acceleration,
}

impl BodyBundle {
    pub fn new(mass: f32, pos: Vec3, vel: Vec3) -> Self {
        Self {
            mass: Mass(mass),
            transform: Transform::from_translation(pos),
            vel: Velocity(vel),
            acc: Acceleration::default(),
        }
    }
}
