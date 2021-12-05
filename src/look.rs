use crate::mouse::*;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct LookDirection {
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl Default for LookDirection {
    fn default() -> Self {
        Self {
            forward: Vec3::Z,
            right: -Vec3::X,
            up: Vec3::Y,
        }
    }
}

#[derive(Debug)]
pub struct LookEntity(pub Entity);

pub fn forward_up(settings: Res<MouseSettings>, mut query: Query<&mut LookDirection>) {
    for mut look in query.iter_mut() {
        let rotation = Quat::from_rotation_ypr(
            settings.yaw_pitch_roll.x,
            settings.yaw_pitch_roll.y,
            settings.yaw_pitch_roll.z,
        );
        look.forward = rotation * -Vec3::Z;
        look.right = rotation * Vec3::X;
        look.up = rotation * Vec3::Y;
    }
}
