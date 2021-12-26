pub mod events;
pub mod render_to_texture;
pub mod tags;

use add_shader::setup_pipeline;
pub use render_to_texture::{
    add_render_to_texture_graph, FIRST_PASS_CAMERA, RENDER_TEXTURE_HANDLE,
};

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{ActiveCameras, Camera, CameraProjection},
        pipeline::{
            BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite, CompareFunction,
            DepthBiasState, DepthStencilState, PipelineDescriptor, RenderPipeline,
            StencilFaceState, StencilState,
        },
        render_graph::{
            self,
            base::{self, MainPass},
            AssetRenderResourcesNode, RenderGraph,
        },
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
        texture::{Extent3d, TextureFormat},
    },
    window::WindowId,
};
use events::BillboardingTranslationEvent;

mod texture_node;
use tags::{FirstPass, FirstPassCube, MainPassCube};
pub use texture_node::TextureNode;

mod add_shader;

pub use add_shader::PIPELINE_HANDLE;

#[derive(Default)]
pub struct DynamicBillboardingPlugin;

impl Plugin for DynamicBillboardingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<BillboardingTranslationEvent>()
            .add_asset::<MyMaterial>()
            .add_startup_system(setup_simple.system());
        // .add_startup_system(setup_pipeline.system())
        // .add_startup_system(setup_simple.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
struct MyMaterial {
    pub color: Color,
}

fn setup_simple(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut shaders: ResMut<Assets<Shader>>,
    // mut materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    mut active_cameras: ResMut<ActiveCameras>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = Extent3d::new(2048, 2048, 1);
add_render_to_texture_graph(&mut render_graph, size);

    asset_server.watch_for_changes().unwrap();

    let pipeline = PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("billboard.vert"),
        fragment: Some(asset_server.load::<Shader, _>("billboard.frag")),
    });

    // let pipeline = PipelineDescriptor::default_config(ShaderStages {
    //     vertex: asset_server.load::<Shader, _>("billboard.vert"),
    //     fragment: None,
    // });

    // let pipeline = PipelineDescriptor {
    //     depth_stencil: Some(DepthStencilState {
    //         format: TextureFormat::Depth32Float,
    //         depth_write_enabled: true,
    //         depth_compare: CompareFunction::Less,
    //         stencil: StencilState {
    //             front: StencilFaceState::IGNORE,
    //             back: StencilFaceState::IGNORE,
    //             read_mask: 0,
    //             write_mask: 0,
    //         },
    //         bias: DepthBiasState {
    //             constant: 0,
    //             slope_scale: 0.0,
    //             clamp: 0.0,
    //         },
    //         clamp_depth: false,
    //     }),
    //     color_target_states: vec![ColorTargetState {
    //         format: TextureFormat::default(),
    //         color_blend: BlendState {
    //             src_factor: BlendFactor::SrcAlpha,
    //             dst_factor: BlendFactor::OneMinusSrcAlpha,
    //             operation: BlendOperation::Add,
    //         },
    //         alpha_blend: BlendState {
    //             src_factor: BlendFactor::One,
    //             dst_factor: BlendFactor::One,
    //             operation: BlendOperation::Add,
    //         },
    //         write_mask: ColorWrite::ALL,
    //     }],
    //     ..PipelineDescriptor::new(ShaderStages {
    //         vertex: shaders.add(Shader::from_glsl(
    //             ShaderStage::Vertex,
    //             include_str!("billboard.vert"),
    //         )),
    //         fragment: Some(shaders.add(Shader::from_glsl(
    //             ShaderStage::Fragment,
    //             include_str!("billboard.frag"),
    //         ))),
    //     })
    // };

    let pipeline_handle = pipelines.add(pipeline);

    // camera
    let mut first_pass_camera = PerspectiveCameraBundle {
        camera: Camera {
            name: Some(FIRST_PASS_CAMERA.to_string()),
            window: WindowId::new(), // otherwise it will use main window size / aspect for calculation of projection matrix
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
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

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        // base_color: Color::BLUE,
        reflectance: 0.0,
        unlit: true,
        ..Default::default()
    });

    // render_graph.add_system_node(
    //     "my_material",
    //     AssetRenderResourcesNode::<MyMaterial>::new(true),
    // );

    // render_graph
    //     .add_node_edge("my_material", base::node::MAIN_PASS)
    //     .unwrap();

    // let material = materials.add(MyMaterial {
    //     color: Color::rgb(0.0, 0.8, 0.0),
    // });

    let cube_size = 695_508.0 * 0.0001 * 0.1 * 2.0;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::ONE * cube_size, flip: false })),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle,
        )]),
        material: material_handle,
        transform: Transform::from_xyz(15.0, 0.0, 0.0),
        visible: Visible {
            is_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    });
    // .insert(material);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    mut active_cameras: ResMut<ActiveCameras>,
) {
    // let size = Extent3d::new(2048, 2048, 1);
    // add_render_to_texture_graph(&mut render_graph, size);

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
    // let mut first_pass_camera = PerspectiveCameraBundle {
    //     camera: Camera {
    //         name: Some(FIRST_PASS_CAMERA.to_string()),
    //         window: WindowId::new(), // otherwise it will use main window size / aspect for calculation of projection matrix
    //         ..Default::default()
    //     },
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 84.0))
    //         .looking_at(Vec3::default(), Vec3::Y),
    //     ..Default::default()
    // };
    // active_cameras.add(FIRST_PASS_CAMERA);

    // let camera_projection = &mut first_pass_camera.perspective_projection;
    // camera_projection.update(size.width as f32, size.height as f32);
    // first_pass_camera.camera.projection_matrix = camera_projection.get_projection_matrix();
    // first_pass_camera.camera.depth_calculation = camera_projection.depth_calculation();

    // commands.spawn_bundle(first_pass_camera);

    // let texture_handle: Handle<Texture> = RENDER_TEXTURE_HANDLE.typed();

    let cube_size = 695_508.0 * 0.0001 * 0.1 * 2.0;
    let mut mesh = Mesh::from(shape::Quad::new(Vec2::ONE * cube_size));

    // mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, {
    //     let mut color: Vec<[f32; 4]> = vec![];
    //     color.resize(4, [1.0; 4]);
    //     color
    // });

    // mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, {
    //     let mut color: Vec<[f32; 4]> = vec![];
    //     color.resize(4, [1.0; 4]);
    //     println!("{}", color);
    //     color
    // });

    let cube_handle = meshes.add(mesh);
    // let cube_handle = meshes.add(Mesh::from(shape::Box::new(cube_size, cube_size, cube_size)));

    let material_handle = materials.add(StandardMaterial {
        // base_color_texture: Some(texture_handle),
        base_color: Color::BLUE,
        reflectance: 0.0,
        unlit: false,
        ..Default::default()
    });

    // add entities to the world
    commands
        .spawn_bundle(PbrBundle {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                PIPELINE_HANDLE.typed(),
            )]),
            mesh: cube_handle,
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(15.0, 0.0, 0.0),
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
    //  u   transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
    //         .looking_at(Vec3::default(), Vec3::Y),
    //     ..Default::default()
    // });
}

fn change_scale_with_distance(
    mut translations: EventReader<BillboardingTranslationEvent>,
    mut query: Query<&mut Transform, With<MainPassCube>>,
) {
    // for event in translations.iter() {
    //     for mut transform in query.iter_mut() {
    //         let result = transform.looking_at(event.translation, Vec3::Y);
    //         // println!("{} -> {}", transform.rotation, transform.looking_at(event.translation, -Vec3::Z).rotation);
    //         transform.rotation = result.rotation * Quat::from_rotation_y(std::f32::consts::PI / 5.0);
    //     }
    // }
}
