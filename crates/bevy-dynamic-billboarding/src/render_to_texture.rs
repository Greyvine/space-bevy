use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pass::*,
        render_graph::{base::node::MAIN_PASS, CameraNode, PassNode, RenderGraph},
        texture::*,
    },
};

use crate::{tags::FirstPass, TextureNode};

pub const TEXTURE_NODE: &str = "texure_node";
pub const DEPTH_TEXTURE_NODE: &str = "depth_texure_node";
pub const FIRST_PASS: &str = "first_pass";
pub const FIRST_PASS_CAMERA: &str = "first_pass_camera";

pub const RENDER_TEXTURE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Texture::TYPE_UUID, 13378939762009864029);

pub fn add_render_to_texture_graph(graph: &mut RenderGraph, size: Extent3d) {
    let mut pass_node = PassNode::<&FirstPass>::new(PassDescriptor {
        color_attachments: vec![RenderPassColorAttachmentDescriptor {
            attachment: TextureAttachment::Input("color_attachment".to_string()),
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::NONE),
                store: true,
            },
        }],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
            attachment: TextureAttachment::Input("depth".to_string()),
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }),
        sample_count: 1,
    });

    pass_node.add_camera(FIRST_PASS_CAMERA);

    graph.add_node(FIRST_PASS, pass_node);
    graph.add_system_node(FIRST_PASS_CAMERA, CameraNode::new(FIRST_PASS_CAMERA));
    graph.add_node_edge(FIRST_PASS_CAMERA, FIRST_PASS).unwrap();

    graph.add_node(
        TEXTURE_NODE,
        TextureNode::new(
            TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: Default::default(),
                usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
            },
            Some(SamplerDescriptor::default()),
            Some(RENDER_TEXTURE_HANDLE),
        ),
    );

    graph.add_node(
        DEPTH_TEXTURE_NODE,
        TextureNode::new(
            TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: TextureUsage::OUTPUT_ATTACHMENT | TextureUsage::SAMPLED,
            },
            None,
            None,
        ),
    );

    graph.add_node_edge(TEXTURE_NODE, FIRST_PASS).unwrap();

    graph
        .add_slot_edge(
            TEXTURE_NODE,
            TextureNode::TEXTURE,
            FIRST_PASS,
            "color_attachment",
        )
        .unwrap();

    graph
        .add_slot_edge(
            DEPTH_TEXTURE_NODE,
            TextureNode::TEXTURE,
            FIRST_PASS,
            "depth",
        )
        .unwrap();

    graph.add_node_edge(FIRST_PASS, MAIN_PASS).unwrap();

    graph.add_node_edge("transform", FIRST_PASS).unwrap();
}
