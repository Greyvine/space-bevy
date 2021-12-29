use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use events::OrginRebasingEvent;

mod events;

#[derive(Default, Inspectable)]
pub struct SimulationCoordinates {
    local_translation: Vec3,
    solar_coordinates: Vec3,
}

#[derive(Default)]
pub struct OriginRebasingPlugin;
pub struct PlayerTag;
pub struct NonPlayerTag;

impl Plugin for OriginRebasingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<OrginRebasingEvent>()
            .add_system(sync_simulation_coordinates.system())
            .add_system(rebase_origin.system());
    }
}

const MAX_BOUND: f32 = 100.0;

fn sync_simulation_coordinates(
    mut rebase_events: EventWriter<OrginRebasingEvent>,
    mut query: Query<(&mut Transform, &mut SimulationCoordinates), With<PlayerTag>>,
) {
    for (mut transform, mut simulation_transform) in query.iter_mut() {
        simulation_transform.local_translation = transform.translation;
        if transform.translation.z < -MAX_BOUND {
            transform.translation.z += 2.0 * MAX_BOUND;
            simulation_transform.local_translation.z += 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.z += 1.0;
            let shift = 2.0 * Vec3::Z * MAX_BOUND;
            rebase_events.send(OrginRebasingEvent::new(&shift));
        }
        if transform.translation.z > MAX_BOUND {
            transform.translation.z -= 2.0 * MAX_BOUND;
            simulation_transform.local_translation.z -= 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.z -= 1.0;
            let shift = -2.0 * Vec3::Z * MAX_BOUND;
            rebase_events.send(OrginRebasingEvent::new(&shift));
        }
    }
}

fn rebase_origin(
    mut events: EventReader<OrginRebasingEvent>,
    mut query: Query<&mut Transform, With<NonPlayerTag>>,
) {
    for event in events.iter() {
        println!("{}", event.translation);
        for mut transform in query.iter_mut() {
            println!("transform -> {}", event.translation);
            transform.translation += event.translation;
            // let distance = (translation).distance(transform.translation);
            // if distance > 500.0 {
            //     let scaling_factor = get_scaling_factor(distance);
            //     transform.scale = Vec3::new(scaling_factor, scaling_factor, scaling_factor);
            // }
        }
    }
}
