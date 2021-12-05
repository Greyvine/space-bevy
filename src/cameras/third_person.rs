use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::cameras::event::*;
use crate::cameras::tag::*;
use crate::look::*;

pub struct ThirdPersonCameraPlugin;

impl Plugin for ThirdPersonCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MouseSettings>()
            .add_event::<PitchEvent>()
            .add_event::<YawEvent>()
            .add_system(handle_mouse_input.system())
            .add_system(handle_yaw.system())
            .add_system(handle_pitch.system());
    }
}

const PITCH_BOUND: f32 = std::f32::consts::FRAC_PI_2 - 1E-3;

fn handle_mouse_input(
    mut settings: ResMut<MouseSettings>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut yaw_events: EventWriter<YawEvent>,
    mut pitch_events: EventWriter<PitchEvent>,
) {
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion_events.iter() {
        // NOTE: -= to invert
        delta -= motion.delta;
    }

    if delta.length_squared() > 1E-6 {
        delta *= settings.sensitivity;
        settings.yaw_pitch_roll += delta.extend(0.0);
        if settings.yaw_pitch_roll.y > PITCH_BOUND {
            settings.yaw_pitch_roll.y = PITCH_BOUND;
        }
        if settings.yaw_pitch_roll.y < -PITCH_BOUND {
            settings.yaw_pitch_roll.y = -PITCH_BOUND;
        }
        pitch_events.send(PitchEvent::new(settings.yaw_pitch_roll.y));
        yaw_events.send(YawEvent::new(settings.yaw_pitch_roll.x));
    }
}

pub fn handle_yaw(mut yaws: EventReader<YawEvent>, mut query: Query<&mut Transform, With<YawTag>>) {
    if let Some(yaw) = yaws.iter().next() {
        for mut transform in query.iter_mut() {
            transform.rotation = Quat::from_rotation_y(**yaw);
        }
    }
}

pub fn handle_pitch(
    mut pitches: EventReader<PitchEvent>,
    mut query: Query<&mut Transform, With<HeadTag>>,
) {
    if let Some(pitch) = pitches.iter().next() {
        for mut transform in query.iter_mut() {
            transform.rotation = Quat::from_rotation_ypr(0.0, **pitch, 0.0);
        }
    }
}
