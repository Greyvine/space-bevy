pub mod render_to_texture;

pub mod tags;

pub use render_to_texture::{add_render_to_texture_graph, FIRST_PASS_CAMERA, RENDER_TEXTURE_HANDLE};

use bevy::{prelude::*, render::{render_graph::{RenderGraph, base::MainPass}, camera::{ActiveCameras, Camera, CameraProjection}, texture::Extent3d}, window::WindowId};

mod texture_node;
use tags::{FirstPass, FirstPassCube, MainPassCube};
pub use texture_node::TextureNode;

#[derive(Default)]
pub struct DynamicBillboardingPlugin;

impl Plugin for DynamicBillboardingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    mut active_cameras: ResMut<ActiveCameras>,
) {
    let size = Extent3d::new(512, 512, 1);
    add_render_to_texture_graph(&mut render_graph, size);

    // let cube_handle = meshes.add(Mesh::from(shape::Cube { size: 4.0 }));
    // let cube_material_handle = materials.add(StandardMaterial {
    //     base_color: Color::rgb(0.8, 0.7, 0.6),
    //     reflectance: 0.02,
    //     roughness: 1.0,
    //     unlit: false,
    //     ..Default::default()
    // });

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: cube_handle,
    //         material: cube_material_handle,
    //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    //         ..Default::default()
    //     })
    //     .insert(FirstPassCube)
    //     .insert(FirstPass)
    //     .remove::<MainPass>();

    // // light
    // // note: currently lights are shared between passes!
    // commands.spawn_bundle(LightBundle {
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    //     ..Default::default()
    // });

    // camera
    let mut first_pass_camera = PerspectiveCameraBundle {
        camera: Camera {
            name: Some(FIRST_PASS_CAMERA.to_string()),
            window: WindowId::new(), // otherwise it will use main window size / aspect for calculation of projection matrix
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    };
    active_cameras.add(FIRST_PASS_CAMERA);

    let camera_projection = &mut first_pass_camera.perspective_projection;
    camera_projection.update(size.width as f32, size.height as f32);
    first_pass_camera.camera.projection_matrix = camera_projection.get_projection_matrix();
    first_pass_camera.camera.depth_calculation = camera_projection.depth_calculation();

    commands.spawn_bundle(first_pass_camera);

    let texture_handle: Handle<Texture> = RENDER_TEXTURE_HANDLE.typed();

    let cube_size = 5.0;
    let cube_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(cube_size, cube_size))));

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        reflectance: 0.02,
        unlit: false,
        ..Default::default()
    });

    // add entities to the world
    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(10.0, 0.0, 1.5),
                rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Billboard"))
        .insert(MainPassCube);

    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
    //         .looking_at(Vec3::default(), Vec3::Y),
    //     ..Default::default()
    // });
}