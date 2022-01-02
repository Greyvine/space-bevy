use std::cmp::{self, min};

use bevy::{math::DVec3, prelude::*};
use bevy_inspector_egui::Inspectable;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin, Line};
use events::OrginRebasingEvent;

mod events;

const MAX_BOUND: f32 = 1_000.0;
const MAX_VIEW: f32 = 10.0 * MAX_BOUND;

#[derive(Default, Inspectable, Clone, Copy)]
pub struct SimulationCoordinates {
    local_translation: Vec3,
    solar_coordinates: Vec3,
    world_coordinates: Vec3,
}

impl SimulationCoordinates {
    pub fn from(pos: Vec3) -> Self {
        Self {
            local_translation: Vec3::new(pos.x % MAX_BOUND, pos.y % MAX_BOUND, pos.z % MAX_BOUND),
            solar_coordinates: Vec3::new(0.0, 0.0, (pos.z / (2.0 * MAX_BOUND)).ceil()),
            world_coordinates: pos,
        }
    }

    pub fn get_relative_render_position(&self, s: SimulationCoordinates) -> Vec3 {
        let result = (self.solar_coordinates * MAX_BOUND + self.local_translation)
            - (s.solar_coordinates * MAX_BOUND + s.local_translation);
        println!(
            "{},{} - {},{} => {}",
            self.solar_coordinates,
            self.local_translation,
            s.solar_coordinates,
            s.local_translation,
            result
        );
        result
        // Self {
        //     local_translation: Vec3::new(pos.x % MAX_BOUND, pos.y % MAX_BOUND, pos.z % MAX_BOUND),
        //     solar_coordinates: Vec3::new(0.0, 0.0, (pos.z / (2.0 * MAX_BOUND)).ceil()),
        //     world_coordinates: pos,
        // }
    }
}

#[derive(Inspectable, Bundle, Default)]
pub struct SimulationBundle {
    transform: Transform,
    simulation_coordinates: SimulationCoordinates,
}

impl SimulationBundle {
    pub fn new(pos: Vec3) -> Self {
        Self {
            transform: Transform::from_translation(pos),
            simulation_coordinates: SimulationCoordinates::from(pos),
        }
    }
    pub fn new_scaled(pos: Vec3) -> Self {
        let scaling_factor = get_scaling_factor(pos.length());
        // println!("sf -> 500.0 / {} = {}", pos.length(), scaling_factor);
        let render_pos = Vec3::new(get_bound(pos.x), get_bound(pos.y), get_bound(pos.z));
        Self {
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(scaling_factor),
                Quat::IDENTITY,
                render_pos,
            )),
            simulation_coordinates: SimulationCoordinates::from(pos),
        }
    }
}

fn get_scaling_factor(distance: f32) -> f32 {
    MAX_VIEW / distance
}

fn get_bound(x: f32) -> f32 {
    if x < 0.0 {
        x.max(-MAX_VIEW)
    } else {
        x.min(MAX_VIEW)
    }
}

#[derive(Default)]
pub struct OriginRebasingPlugin;

pub struct PlayerTag;

pub struct NonPlayerTag;

pub struct BillieTag;

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

fn sync_simulation_coordinates(
    mut rebase_events: EventWriter<OrginRebasingEvent>,
    mut q: QuerySet<(
        Query<(&Transform, &mut SimulationCoordinates), With<PlayerTag>>,
        Query<(&mut Transform, &SimulationCoordinates), With<SimulationCoordinates>>,
        Query<&mut SimulationCoordinates, With<NonPlayerTag>>,
    )>,
) {
    let mut shift = Vec3::ZERO;

    let (transform, mut simulation_transform) = q
        .q0_mut()
        .single_mut()
        .expect("There should always be exactly one player in the game!");

    simulation_transform.local_translation = transform.translation;
    if transform.translation.x < -MAX_BOUND {
        simulation_transform.local_translation.x += 2.0 * MAX_BOUND;
        simulation_transform.solar_coordinates.x -= 1.0;
        shift = 2.0 * Vec3::X * MAX_BOUND;
        rebase_events.send(OrginRebasingEvent::new(&shift));
    } else if transform.translation.x > MAX_BOUND {
        simulation_transform.local_translation.x -= 2.0 * MAX_BOUND;
        simulation_transform.solar_coordinates.x += 1.0;
        shift = -2.0 * Vec3::X * MAX_BOUND;
        rebase_events.send(OrginRebasingEvent::new(&shift));
    }
    if transform.translation.y < -MAX_BOUND {
        simulation_transform.local_translation.y += 2.0 * MAX_BOUND;
        simulation_transform.solar_coordinates.y -= 1.0;
        shift = 2.0 * Vec3::Y * MAX_BOUND;
        rebase_events.send(OrginRebasingEvent::new(&shift));
    } else if transform.translation.y > MAX_BOUND {
        simulation_transform.local_translation.y -= 2.0 * MAX_BOUND;
        simulation_transform.solar_coordinates.y += 1.0;
        shift = -2.0 * Vec3::Y * MAX_BOUND;
        rebase_events.send(OrginRebasingEvent::new(&shift));
    }
    if transform.translation.z < -MAX_BOUND {
        simulation_transform.local_translation.z += 2.0 * MAX_BOUND;
        simulation_transform.solar_coordinates.z -= 1.0;
        shift = 2.0 * Vec3::Z * MAX_BOUND;
        rebase_events.send(OrginRebasingEvent::new(&shift));
    } else if transform.translation.z > MAX_BOUND {
        simulation_transform.local_translation.z -= 2.0 * MAX_BOUND;
        simulation_transform.solar_coordinates.z += 1.0;
        shift = -2.0 * Vec3::Z * MAX_BOUND;
        rebase_events.send(OrginRebasingEvent::new(&shift));
    }

    let a = simulation_transform.solar_coordinates.clone();
    let b = simulation_transform.clone();

    if shift != Vec3::ZERO {
        for (mut transform, npc_simulation_coordinates) in q.q1_mut().iter_mut() {
            // let distance = npc_simulation_coordinates.solar_coordinates.distance(a) * MAX_BOUND;
            // if distance < MAX_VIEW {
            transform.translation += shift;
            // }
            // else {
            //     let scaling_factor = get_scaling_factor(distance).min(1.0);
            //     let relative_render_position = transform.translation - npc_simulation_coordinates.get_relative_render_position(b);
            //     transform.translation += relative_render_position * scaling_factor;
            //     transform.scale = Vec3::splat(scaling_factor);
            // }
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
        Vec3::new(-MAX_BOUND, MAX_BOUND, -MAX_BOUND),
        Vec3::new(MAX_BOUND, MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::RED,
        Color::RED,
    );
    let line2 = Line::new(
        Vec3::new(-MAX_BOUND, MAX_BOUND, MAX_BOUND),
        Vec3::new(MAX_BOUND, MAX_BOUND, MAX_BOUND),
        0.0,
        Color::GREEN,
        Color::GREEN,
    );
    let line3 = Line::new(
        Vec3::new(-MAX_BOUND, MAX_BOUND, MAX_BOUND),
        Vec3::new(-MAX_BOUND, MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::BLUE,
        Color::BLUE,
    );
    let line4 = Line::new(
        Vec3::new(MAX_BOUND, MAX_BOUND, MAX_BOUND),
        Vec3::new(MAX_BOUND, MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::WHITE,
        Color::WHITE,
    );
    lines.user_lines.push(line1);
    lines.user_lines.push(line2);
    lines.user_lines.push(line3);
    lines.user_lines.push(line4);

    let line1 = Line::new(
        Vec3::new(-MAX_BOUND, -MAX_BOUND, -MAX_BOUND),
        Vec3::new(MAX_BOUND, -MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::RED,
        Color::RED,
    );
    let line2 = Line::new(
        Vec3::new(-MAX_BOUND, -MAX_BOUND, MAX_BOUND),
        Vec3::new(MAX_BOUND, -MAX_BOUND, MAX_BOUND),
        0.0,
        Color::GREEN,
        Color::GREEN,
    );
    let line3 = Line::new(
        Vec3::new(-MAX_BOUND, -MAX_BOUND, MAX_BOUND),
        Vec3::new(-MAX_BOUND, -MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::BLUE,
        Color::BLUE,
    );
    let line4 = Line::new(
        Vec3::new(MAX_BOUND, -MAX_BOUND, MAX_BOUND),
        Vec3::new(MAX_BOUND, -MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::WHITE,
        Color::WHITE,
    );
    lines.user_lines.push(line1);
    lines.user_lines.push(line2);
    lines.user_lines.push(line3);
    lines.user_lines.push(line4);

    let line1 = Line::new(
        Vec3::new(-MAX_BOUND, MAX_BOUND, -MAX_BOUND),
        Vec3::new(-MAX_BOUND, -MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::RED,
        Color::RED,
    );
    let line2 = Line::new(
        Vec3::new(MAX_BOUND, MAX_BOUND, -MAX_BOUND),
        Vec3::new(MAX_BOUND, -MAX_BOUND, -MAX_BOUND),
        0.0,
        Color::GREEN,
        Color::GREEN,
    );
    let line3 = Line::new(
        Vec3::new(-MAX_BOUND, MAX_BOUND, MAX_BOUND),
        Vec3::new(-MAX_BOUND, -MAX_BOUND, MAX_BOUND),
        0.0,
        Color::BLUE,
        Color::BLUE,
    );
    let line4 = Line::new(
        Vec3::new(MAX_BOUND, MAX_BOUND, MAX_BOUND),
        Vec3::new(MAX_BOUND, -MAX_BOUND, MAX_BOUND),
        0.0,
        Color::WHITE,
        Color::WHITE,
    );
    lines.user_lines.push(line1);
    lines.user_lines.push(line2);
    lines.user_lines.push(line3);
    lines.user_lines.push(line4);
}
