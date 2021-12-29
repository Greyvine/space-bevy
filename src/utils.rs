use crate::cameras::tag::*;
use crate::controllers::tag::*;
use crate::look::*;
use crate::scale::{convert_metres_to_units, M_TO_UNIT_SCALE};
use bevy::prelude::*;
use bevy::render::camera::{Camera, PerspectiveProjection};
use bevy::render::wireframe::Wireframe;
use bevy_dynamic_billboarding::FIRST_PASS_CAMERA;
use bevy_origin_rebasing::{SimulationCoordinates, PlayerTag, NonPlayerTag};
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

    // let pos = Vec3::new(
    //     -8.873674344461769E-01,
    //     -4.697992257377307E-01,
    //     2.381003809013169E-05,
    // ) * AU_TO_UNIT_SCALE;

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

pub fn spawn_marker(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::TOMATO.into(),
                roughness: 0.6,
                ..Default::default()
            }),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::splat(1.0),
                Quat::IDENTITY,
                -Vec3::Z * 2.0,
            )),
            ..Default::default()
        })
        .insert(SimulationCoordinates::default())
        .insert(Wireframe)
        .insert(NonPlayerTag)
        .insert(Name::new("marker"));
}
