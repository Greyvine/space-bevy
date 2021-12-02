use crate::event::*;
use crate::look::*;
use bevy::prelude::*;

pub const INPUT_TO_EVENTS_SYSTEM: &str = "input_to_events";
pub const INPUT_TO_LOOK_SYSTEM: &str = "input_to_look";
pub const FORWARD_UP_SYSTEM: &str = "forward_up";

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ForceEvent>()
            .add_event::<YawEvent>()
            .add_event::<PitchEvent>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_input.system().label(INPUT_TO_EVENTS_SYSTEM),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                input_to_look.system().label(INPUT_TO_LOOK_SYSTEM),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                forward_up
                    .system()
                    .label(FORWARD_UP_SYSTEM)
                    .after(INPUT_TO_EVENTS_SYSTEM)
                    .after(INPUT_TO_LOOK_SYSTEM),
            );
    }
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    look_direction_query: Query<&LookDirection>,
    mut controller_query: Query<&LookEntity>,
    mut force_events: EventWriter<ForceEvent>,
) {
    let xz = Vec3::new(1.0, 0.0, 1.0);

    for look_entity in controller_query.iter_mut() {
        let look = look_direction_query
            .get_component::<LookDirection>(look_entity.0)
            .expect("Failed to get LookDirection from Entity");

        let (forward, right, up) = (
            (look.forward * xz).normalize(),
            (look.right * xz).normalize(),
            Vec3::Y,
        );

        let mut desired_velocity = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            desired_velocity += forward;
        }
        if keys.pressed(KeyCode::S) {
            desired_velocity -= forward;
        }
        if keys.pressed(KeyCode::D) {
            desired_velocity += right;
        }
        if keys.pressed(KeyCode::A) {
            desired_velocity -= right;
        }
        if keys.pressed(KeyCode::Space) {
            desired_velocity += up;
        }
        if keys.pressed(KeyCode::LShift) {
            desired_velocity -= up;
        }

        desired_velocity *= 0.5;

        force_events.send(ForceEvent::new(&desired_velocity))
    }
}

pub struct YawTag;
pub struct CameraTag;
pub struct BodyTag;
pub struct HeadTag;

pub fn controller_to_yaw(
    mut yaws: EventReader<YawEvent>,
    mut query: Query<&mut Transform, With<YawTag>>,
) {
    if let Some(yaw) = yaws.iter().next() {
        for mut transform in query.iter_mut() {
            transform.rotation = Quat::from_rotation_y(**yaw);
        }
    }
}

pub fn controller_to_pitch(
    mut pitches: EventReader<PitchEvent>,
    mut query: Query<&mut Transform, With<HeadTag>>,
) {
    if let Some(pitch) = pitches.iter().next() {
        for mut transform in query.iter_mut() {
            transform.rotation = Quat::from_rotation_ypr(0.0, **pitch, 0.0);
        }
    }
}
