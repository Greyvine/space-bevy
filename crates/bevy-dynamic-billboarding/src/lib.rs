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
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::ONE * cube_size,
            flip: false,
        })),
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
