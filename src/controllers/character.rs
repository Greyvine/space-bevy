use crate::controllers::event::*;
use crate::controllers::tag::*;
use crate::look::*;
use bevy::prelude::*;
use bevy_dynamic_billboarding::events::BillboardingTranslationEvent;
use bevy_dynamic_object_scaling::events::ScalingTranslationEvent;

pub const INPUT_TO_EVENTS_SYSTEM: &str = "input_to_events";
pub const FORWARD_UP_SYSTEM: &str = "forward_up";

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ForceEvent>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                handle_input.system().label(INPUT_TO_EVENTS_SYSTEM),
            )
            .add_system(controller_to_kinematic.system())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                forward_up
                    .system()
                    .label(FORWARD_UP_SYSTEM)
                    .after(INPUT_TO_EVENTS_SYSTEM),
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
        if keys.pressed(KeyCode::Q) {
            desired_velocity += up;
        }
        if keys.pressed(KeyCode::E) {
            desired_velocity -= up;
        }

        let speed = if keys.pressed(KeyCode::LShift) {
            200.0
        } else {
            0.5
        };

        desired_velocity *= speed;

        force_events.send(ForceEvent::new(&desired_velocity))
    }
}

fn controller_to_kinematic(
    mut translations: EventReader<ForceEvent>,
    mut query: Query<&mut Transform, With<BodyTag>>,
    mut scale_events: EventWriter<ScalingTranslationEvent>,
    mut billboarding_events: EventWriter<BillboardingTranslationEvent>,
) {
    for mut transform in query.iter_mut() {
        for translation in translations.iter() {
            transform.translation += **translation;
        }
        scale_events.send(ScalingTranslationEvent::new(&transform.translation));
        billboarding_events.send(BillboardingTranslationEvent::new(&transform.translation));
        // NOTE: This is just an example to stop falling past the initial body height
        // With a physics engine you would indicate that the body has collided with
        // something and should stop, depending on how your game works.
        // if transform.translation.y < 0.0 {
        //     transform.translation.y = 0.0;
        //     // controller.jumping = false;
        // }
    }
}
