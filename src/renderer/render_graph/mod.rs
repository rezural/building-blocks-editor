mod lights_node;
mod pbr_pipeline;

use bevy::ecs::world::World;
pub use lights_node::*;
pub use pbr_pipeline::*;

/// the names of pbr graph nodes
pub mod node {
    pub const TRANSFORM: &str = "transform";
    pub const ARRAY_MATERIAL: &str = "array_material";
    pub const LIGHTS: &str = "lights";
}

/// the names of pbr uniforms
pub mod uniform {
    pub const LIGHTS: &str = "Lights";
}

use crate::renderer::ArrayMaterial;
use bevy::asset::Assets;
use bevy::render::{
    pipeline::PipelineDescriptor,
    render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
    shader::Shader,
};
use bevy::transform::prelude::GlobalTransform;

pub(crate) fn add_pbr_graph(world: &mut World) {
    {
        let mut graph = world.get_resource_mut::<RenderGraph>().unwrap();
        graph.add_system_node(
            node::TRANSFORM,
            RenderResourcesNode::<GlobalTransform>::new(true),
        );
        graph.add_system_node(
            node::ARRAY_MATERIAL,
            AssetRenderResourcesNode::<ArrayMaterial>::new(true),
        );
        graph.add_system_node(node::LIGHTS, LightsNode::new(10));

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
    let pipeline = build_pbr_pipeline(&mut world.get_resource_mut::<Assets<Shader>>().unwrap());
    let mut pipelines = world
        .get_resource_mut::<Assets<PipelineDescriptor>>()
        .unwrap();
    pipelines.set_untracked(PBR_PIPELINE_HANDLE, pipeline);
}
