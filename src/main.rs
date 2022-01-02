// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::system::exit_on_esc_system;
use bevy::pbr::AmbientLight;
// use bevy::pbr::AmbientLight;
use bevy::prelude::*;
use bevy::render::wireframe::WireframePlugin;
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_dynamic_billboarding::DynamicBillboardingPlugin;
use bevy_dynamic_object_scaling::DynamicObjectScalingPlugin;
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};
use bevy_origin_rebasing::{OriginRebasingPlugin, SimulationBundle, SimulationCoordinates};
use space::cameras::third_person::*;
use space::controllers::character::*;
use space::gravity::resources::Gravity;
use space::spawn::planets::*;
use space::utils::*;

fn main() {
    App::build()
        .insert_resource(CharacterSettings {
            focal_point: -Vec3::Z,
            follow_offset: Vec3::new(0.0, 1.0, 8.0),
            head_yaw: 0.0,
            ..Default::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .insert_resource(InspectableRegistry::default().with::<SimulationCoordinates>())
        .init_resource::<Gravity>()
        .init_resource::<CharacterSettings>()
        .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterControllerPlugin)
        .add_plugin(ThirdPersonCameraPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(exit_on_esc_system.system())
        // .add_startup_system(spawn_lights.system())
        .add_startup_system(spawn_character.system())
        // .add_startup_system(spawn_earth.system())
        .add_startup_system(spawn_earth_billboard.system())
        // .add_startup_system(spawn_marker.system())
        // .add_startup_system(spawn_marker_billboard.system())
        // .add_startup_system(spawn_world.system())
        // .add_startup_system(spawn_planets.system())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DynamicObjectScalingPlugin)
        .add_plugin(DynamicBillboardingPlugin)
        .add_plugin(OriginRebasingPlugin)
        .add_plugin(WireframePlugin)
        .run();
}
