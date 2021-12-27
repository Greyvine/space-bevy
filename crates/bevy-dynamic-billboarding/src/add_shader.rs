use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::*,
        render_graph::{base, RenderGraph, RenderResourcesNode},
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
};

pub const PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x936896ad9d35720c_u64);

pub(crate) fn setup_pipeline(
    // mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
) {

    // let pipeline = PipelineDescriptor::default_config( ShaderStages {
    //             vertex: shaders.add(Shader::from_glsl(
    //                 ShaderStage::Vertex,
    //                 include_str!("billboard.vert"),
    //             )),
    //             fragment: None,
    //         });

    // let pipeline = PipelineDescriptor {
    //     name: None,
    //     layout: None,
    //     shader_stages: ShaderStages {
    //         vertex: shaders.add(Shader::from_glsl(
    //             ShaderStage::Vertex,
    //             include_str!("billboard.vert"),
    //         )),
    //         fragment: None,
    //     },
    //     primitive: PrimitiveState {
    //         topology: PrimitiveTopology::TriangleList,
    //         strip_index_format: None,
    //         front_face: FrontFace::Ccw,
    //         cull_mode: CullMode::Back,
    //         polygon_mode: PolygonMode::Fill,
    //     },
    //     depth_stencil: Some(DepthStencilState {
    //         format: TextureFormat::Depth32Float,
    //         depth_write_enabled: false,
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
    //     multisample: MultisampleState {
    //         count: 1,
    //         mask: !0,
    //         alpha_to_coverage_enabled: false,
    //     },
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
    // };

    // pipelines.set_untracked(PIPELINE_HANDLE, pipeline);
}
