use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use space::controller::*;
use space::log::*;
use space::look::*;
use space::utils::*;

fn main() {
    App::build()
        .insert_resource(CharacterSettings {
            focal_point: -Vec3::Z,
            follow_offset: Vec3::new(0.0, 1.0, 8.0),
            head_yaw: 0.0,
            ..Default::default()
        })
        .init_resource::<CharacterSettings>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<MouseSettings>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ControllerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(exit_on_esc_system.system())
        .add_startup_system(spawn_lights.system())
        .add_startup_system(spawn_character.system())
        .add_startup_system(spawn_world.system())
        .add_system(controller_to_kinematic.system())
        .add_system(controller_to_yaw.system())
        // .add_system(controller_to_pitch.system())
        .run();
}

// use bevy_flycam::PlayerPlugin;

// struct Person;
// struct Name(String);
// struct GreetTimer(Timer);

// fn add_people(mut commands: Commands) {
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Elaina Proctor".to_string()));
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Renzo Hume".to_string()));
//     commands
//         .spawn()
//         .insert(Person)
//         .insert(Name("Zayna Nieves".to_string()));
// }

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// pub struct HelloPlugin;

// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
//             .add_startup_system(add_people.system())
//             .add_system(greet_people.system());
//     }
// }

// .add_startup_system(setup.system())
// .add_plugin(PlayerPlugin)

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn_bundle(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Icosphere {
//             radius: 0.05,
//             subdivisions: 79,
//         })),
//         material: materials.add(Color::rgb(0.4, 0.95, 1.0).into()),
//         ..Default::default()
//     });

//     // Sun
//     commands.spawn_bundle(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Icosphere {
//             radius: 0.5,
//             subdivisions: 79,
//         })),
//         material: materials.add(StandardMaterial {
//             base_color: Color::hex("ffffff").unwrap(),
//             metallic: 1.0,
//             roughness: 0.0,
//             emissive: Color::hex("ffffff").unwrap(),
//             ..Default::default()
//         }),
//         transform: Transform::from_xyz(19.8, 0.0, 0.0),
//         ..Default::default()
//     });

//     // light
//     commands.spawn_bundle(LightBundle {
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..Default::default()
//     });

//     // camera
//     commands.spawn_bundle(PerspectiveCameraBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     });
// }
