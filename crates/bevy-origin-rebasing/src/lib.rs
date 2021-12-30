use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin, Line};
use events::OrginRebasingEvent;

mod events;

#[derive(Default, Inspectable)]
pub struct SimulationCoordinates {
    local_translation: Vec3,
    solar_coordinates: Vec3,
}

#[derive(Inspectable, Bundle)]
pub struct SimulationBundle {
    transform: Transform,
    simulation_coordinates: SimulationCoordinates,
}

impl SimulationBundle {
    pub fn new(pos: Vec3) -> Self {
        Self {
            transform: Transform::from_translation(pos),
            simulation_coordinates: SimulationCoordinates {
                local_translation: Vec3::new(0.0, 0.0, pos.z % MAX_BOUND),
                solar_coordinates: Vec3::new(0.0, 0.0, (pos.z / (2.0 * MAX_BOUND)).ceil()),
            },
        }
    }
}

#[derive(Default)]
pub struct OriginRebasingPlugin;
pub struct PlayerTag;
pub struct NonPlayerTag;

impl Plugin for OriginRebasingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<OrginRebasingEvent>()
            .add_plugin(DebugLinesPlugin)
            .insert_resource(DebugLines {
                depth_test: true,
                ..Default::default()
            })
            .add_startup_system(setup.system())
            .add_system(sync_simulation_coordinates.system())
            .add_system(rebase_origin.system());
    }
}

const MAX_BOUND: f32 = 10.0;

fn sync_simulation_coordinates(
    mut rebase_events: EventWriter<OrginRebasingEvent>,
    mut q: QuerySet<(
        Query<(&Transform, &mut SimulationCoordinates), With<PlayerTag>>,
        Query<&mut Transform, With<SimulationCoordinates>>,
    )>,
) {
    let mut shift = Vec3::ZERO;
    for (transform, mut simulation_transform) in q.q0_mut().iter_mut() {
        simulation_transform.local_translation = transform.translation;
        if transform.translation.z < -MAX_BOUND {
            simulation_transform.local_translation.z += 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.z -= 1.0;
            shift = 2.0 * Vec3::Z * MAX_BOUND;
            rebase_events.send(OrginRebasingEvent::new(&shift));
        }
        if transform.translation.z > MAX_BOUND {
            simulation_transform.local_translation.z -= 2.0 * MAX_BOUND;
            simulation_transform.solar_coordinates.z += 1.0;
            shift = -2.0 * Vec3::Z * MAX_BOUND;
            rebase_events.send(OrginRebasingEvent::new(&shift));
        }
    }
    if shift != Vec3::ZERO {
        for mut transform in q.q1_mut().iter_mut() {
            transform.translation += shift;
        }
    }
}

fn rebase_origin(
    mut events: EventReader<OrginRebasingEvent>,
    mut query: Query<&mut Transform, With<NonPlayerTag>>,
) {
    // for event in events.iter() {
    //     for mut transform in query.iter_mut() {
    //         println!("transform -> {}", event.translation);
    //         transform.translation += event.translation;
    //         // let distance = (translation).distance(transform.translation);
    //         // if distance > 500.0 {
    //         //     let scaling_factor = get_scaling_factor(distance);
    //         //     transform.scale = Vec3::new(scaling_factor, scaling_factor, scaling_factor);
    //         // }
    //     }
    // }
}

fn setup(mut lines: ResMut<DebugLines>) {
    let line1 = Line::new(
        Vec3::new(-MAX_BOUND, 0.0, -MAX_BOUND),
        Vec3::new(MAX_BOUND, 0.0, -MAX_BOUND),
        0.0,
        Color::RED,
        Color::RED,
    );
    let line2 = Line::new(
        Vec3::new(-MAX_BOUND, 0.0, MAX_BOUND),
        Vec3::new(MAX_BOUND, 0.0, MAX_BOUND),
        0.0,
        Color::GREEN,
        Color::GREEN,
    );
    let line3 = Line::new(
        Vec3::new(-MAX_BOUND, 0.0, MAX_BOUND),
        Vec3::new(-MAX_BOUND, 0.0, -MAX_BOUND),
        0.0,
        Color::BLUE,
        Color::BLUE,
    );
    let line4 = Line::new(
        Vec3::new(MAX_BOUND, 0.0, MAX_BOUND),
        Vec3::new(MAX_BOUND, 0.0, -MAX_BOUND),
        0.0,
        Color::WHITE,
        Color::WHITE,
    );

    lines.user_lines.push(line1);
    lines.user_lines.push(line2);
    lines.user_lines.push(line3);
    lines.user_lines.push(line4);
}
