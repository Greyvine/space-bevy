use crate::cameras::tag::*;
use crate::controllers::tag::*;
use crate::look::*;
use crate::scale::{convert_metres_to_units, KM_TO_UNIT_SCALE, M_TO_UNIT_SCALE};
use bevy::prelude::*;
use bevy::render::camera::{ActiveCameras, Camera, CameraProjection, PerspectiveProjection};
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::base::MainPass;
use bevy::render::shader::ShaderStages;
use bevy::render::wireframe::Wireframe;
use bevy::window::WindowId;
use bevy_dynamic_billboarding::tags::{BillboardTag, FirstPass};
use bevy_dynamic_billboarding::{FIRST_PASS_CAMERA, RENDER_TEXTURE_HANDLE};
use bevy_origin_rebasing::{NonPlayerTag, PlayerTag, SimulationBundle, SimulationCoordinates};
use rand::Rng;

pub struct CharacterSettings {
    pub scale: Vec3,
    pub head_scale: f32,
    pub head_yaw: f32,
    pub follow_offset: Vec3,
    pub focal_point: Vec3,
}

impl Default for CharacterSettings {
    fn default() -> Self {
        Self {
            scale: convert_metres_to_units(Vec3::new(0.3, 0.5, 1.9)),
            head_scale: 0.3 * M_TO_UNIT_SCALE,
            head_yaw: 0.0,
            follow_offset: Vec3::new(0.0, 0.0, 0.0), // Relative to head
            focal_point: Vec3::ZERO,                 // Relative to head
        }
    }
}

pub fn spawn_character(
    mut commands: Commands,
    character_settings: Res<CharacterSettings>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let box_y = 1.0;
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let red = materials.add(Color::hex("800000").unwrap().into());

    let body_model = commands
        .spawn_bundle(PbrBundle {
            material: red.clone(),
            mesh: cube.clone(),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                character_settings.scale - character_settings.head_scale * Vec3::Y,
                Quat::IDENTITY,
                Vec3::new(
                    0.0,
                    0.5 * (box_y + character_settings.scale.y - character_settings.head_scale)
                        - 1.695,
                    0.0,
                ),
            )),
            ..Default::default()
        })
        .insert(Wireframe)
        .id();

    let r = Transform::identity();
    // r.translation += pos;

    let body = commands
        .spawn_bundle((GlobalTransform::identity(), r, BodyTag))
        .insert(Name::new("player"))
        .insert(SimulationCoordinates::default())
        .insert(PlayerTag)
        .id();

    let yaw = commands
        .spawn_bundle((GlobalTransform::identity(), Transform::identity(), YawTag))
        .id();

    let head = commands
        .spawn_bundle((
            GlobalTransform::identity(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::ONE,
                Quat::from_rotation_y(character_settings.head_yaw),
                Vec3::new(
                    0.0,
                    0.5 * (box_y - character_settings.head_scale) + character_settings.scale.y
                        - 1.695,
                    0.0,
                ),
            )),
            HeadTag,
        ))
        .id();

    let head_model = commands
        .spawn_bundle(PbrBundle {
            material: red,
            mesh: cube,
            transform: Transform::from_scale(Vec3::splat(character_settings.head_scale)),
            ..Default::default()
        })
        .insert(Wireframe)
        .id();

    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            // camera: Camera {
            //     name: Some(FIRST_PASS_CAMERA.to_string()),
            //     // window: WindowId::new(), // otherwise it will use main window size / aspect for calculation of projection matrix
            //     ..Default::default()
            // },
            transform: Transform::from_matrix(Mat4::face_toward(
                character_settings.follow_offset,
                character_settings.focal_point,
                Vec3::Y,
            )),
            perspective_projection: PerspectiveProjection {
                far: 1_000_000_000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle((LookDirection::default(), CameraTag))
        .id();

    commands
        .entity(body)
        .insert(LookEntity(camera))
        .push_children(&[yaw]);

    commands.entity(yaw).push_children(&[body_model, head]);
    commands.entity(head).push_children(&[head_model, camera]);
}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(-10.0, 0.0, -10.0)),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(20.0, 0.0, -10.0)),
        ..Default::default()
    });
}

pub fn spawn_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Ground cuboid
    let grey = materials.add(Color::hex("808080").unwrap().into());
    commands.spawn_bundle(PbrBundle {
        material: grey,
        mesh: cube.clone(),
        transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(20.0, 1.0, 20.0),
            Quat::IDENTITY,
            -Vec3::Y * 2.0,
        )),
        ..Default::default()
    });

    let teal = materials.add(Color::hex("008080").unwrap().into());
    let cube_scale = 0.25;
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(-10.0..10.0);
        let z = rng.gen_range(-10.0..10.0);
        commands.spawn_bundle(PbrBundle {
            material: teal.clone(),
            mesh: cube.clone(),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(cube_scale),
                Quat::IDENTITY,
                Vec3::new(x, 0.5 * (cube_scale - 1.0), z),
            )),
            ..Default::default()
        });
    }
}

// pub fn spawn_marker(
//     mut commands: Commands,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     commands
//         .spawn_bundle(PbrBundle {
//             material: materials.add(StandardMaterial {
//                 base_color: Color::TOMATO.into(),
//                 roughness: 0.6,
//                 ..Default::default()
//             }),
//             mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//             ..Default::default()
//         })
//         // .insert(SimulationCoordinates::default())
//         .insert_bundle(SimulationBundle::new(-Vec3::Z * 100.0))
//         .insert(Wireframe)
//         .insert(NonPlayerTag)
//         .insert(Name::new("marker"));
// }

pub fn spawn_earth(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let radius = 6051.84 * KM_TO_UNIT_SCALE;
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::NAVY.into(),
                roughness: 0.6,
                ..Default::default()
            }),
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius,
                subdivisions: 20,
            })),
            ..Default::default()
        })
        .insert_bundle(SimulationBundle::new_scaled(-Vec3::Z * radius * 4.0))
        .insert(Wireframe)
        .insert(NonPlayerTag)
        .insert(Name::new("Marker"));
}

pub fn spawn_marker_billboard(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let radius = 6051.84 * KM_TO_UNIT_SCALE;
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::MAROON.into(),
                roughness: 0.6,
                ..Default::default()
            }),
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::splat(2.0 * radius),
                flip: false,
            })),
            ..Default::default()
        })
        .insert_bundle(SimulationBundle::new(-Vec3::Z * radius * 4.0))
        .insert(Wireframe)
        .insert(NonPlayerTag)
        .insert(Name::new("MarkerBillboard"));
}

pub fn spawn_earth_billboard(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut active_cameras: ResMut<ActiveCameras>,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
) {
    let radius = 2.0;
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::NAVY.into(),
                roughness: 0.6,
                ..Default::default()
            }),
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius,
                subdivisions: 20,
            })),
            ..Default::default()
        })
        .insert(Wireframe)
        .insert(NonPlayerTag)
        .insert(BillboardTag)
        .insert(Name::new("Earth"))
        .insert(FirstPass)
        .remove::<MainPass>();

    let mut first_pass_camera = PerspectiveCameraBundle {
        camera: Camera {
            name: Some(FIRST_PASS_CAMERA.to_string()),
            window: WindowId::new(), // otherwise it will use main window size / aspect for calculation of projection matrix
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, radius + 3.5))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    };

    active_cameras.add(FIRST_PASS_CAMERA);

    let camera_projection = &mut first_pass_camera.perspective_projection;
    camera_projection.update(2048.0, 2048.0);
    first_pass_camera.camera.projection_matrix = camera_projection.get_projection_matrix();
    first_pass_camera.camera.depth_calculation = camera_projection.depth_calculation();

    commands.spawn_bundle(first_pass_camera);

    let texture_handle: Handle<Texture> = RENDER_TEXTURE_HANDLE.typed();

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        // base_color: Color::BLUE,
        reflectance: 0.0,
        unlit: true,
        ..Default::default()
    });

    asset_server.watch_for_changes().unwrap();

    let pipeline = PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("billboard.vert"),
        fragment: Some(asset_server.load::<Shader, _>("billboard.frag")),
    });

    let pipeline_handle = pipelines.add(pipeline);

    let radius = 6051.84 * KM_TO_UNIT_SCALE;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::splat(20000.0),
                flip: false,
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            material: material_handle,
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wireframe)
        .insert_bundle(SimulationBundle::new_scaled(-Vec3::Z * radius * 4.0))
        .insert(Name::new("RealBillboard"));
}
