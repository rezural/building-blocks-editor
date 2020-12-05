mod forward_pipeline;
mod lights_node;

pub use forward_pipeline::*;
pub use lights_node::*;

/// the names of graph nodes
pub mod node {
    pub const TRANSFORM: &str = "transform";
    pub const ARRAY_MATERIAL: &str = "standard_material";
    pub const LIGHTS: &str = "lights";
}

/// the names of uniforms
pub mod uniform {
    pub const LIGHTS: &str = "Lights";
}

use super::array_material::ArrayMaterial;

use bevy::asset::Assets;
use bevy::ecs::Resources;
use bevy::render::{
    pipeline::PipelineDescriptor,
    render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
    shader::Shader,
};
use bevy::transform::prelude::GlobalTransform;

pub(crate) fn add_triplanar_render_graph(graph: &mut RenderGraph, resources: &Resources) {
    graph.add_system_node(
        node::TRANSFORM,
        RenderResourcesNode::<GlobalTransform>::new(true),
    );
    graph.add_system_node(
        node::ARRAY_MATERIAL,
        AssetRenderResourcesNode::<ArrayMaterial>::new(true),
    );
    graph.add_system_node(node::LIGHTS, LightsNode::new(10));
    let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
    pipelines.set_untracked(
        FORWARD_PIPELINE_HANDLE,
        build_forward_pipeline(&mut shaders),
    );

    // TODO: replace these with "autowire" groups
    graph
        .add_node_edge(node::ARRAY_MATERIAL, base::node::MAIN_PASS)
        .unwrap();
    graph
        .add_node_edge(node::TRANSFORM, base::node::MAIN_PASS)
        .unwrap();
    graph
        .add_node_edge(node::LIGHTS, base::node::MAIN_PASS)
        .unwrap();
}
