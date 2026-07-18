pub use wgpu::{
    ComputePassDescriptor, ComputePipeline, RenderPassDescriptor, RenderPipeline
};

use wgpu::{CommandEncoder, Queue};

use crate::{
    assets::RenderAssets,
    core::{RenderObjectError, RenderObjectResult, SwapchainTexture},
    resources::RenderResources
};


#[derive(Default)]
pub(crate) struct RenderGraph {
    nodes: Vec<Box<dyn RenderNode>>
}

impl RenderGraph {
    pub fn run(
        &self,
        frame: &mut FrameContext,
        assets: &RenderAssets,
        resources: &RenderResources
    ) {
        self.nodes
            .iter()
            .for_each(|node| node.run(frame, assets, resources));
    }

    pub fn add_node<R: RenderNode>(&mut self, node: R)
    where R: 'static {
        self.nodes.push(Box::new(node))
    }
}


pub trait RenderNode {
    fn run(
        &self,
        frame: &mut FrameContext,
        assets: &RenderAssets,
        resources: &RenderResources
    );
}


pub struct RenderGraphCreation<'renderer> {
    graph: &'renderer mut RenderGraph,
    assets: &'renderer RenderAssets
}

impl<'r> RenderGraphCreation<'r> {
    pub(crate) fn new(graph: &'r mut RenderGraph, assets: &'r RenderAssets) -> Self {
        Self { graph, assets }
    }

    #[allow(private_bounds)]
    pub fn add_node<R: RenderNode>(&mut self) -> RenderObjectResult<'r, ()>
    where R: TryFrom<&'r RenderAssets, Error = RenderObjectError<'r>> + 'static {
        let node: R = self.assets.try_into()?;
        self.graph.add_node(node);
        Ok(())
    }
}


pub struct FrameContext {
    pub encoder: CommandEncoder,
    pub texture: SwapchainTexture
}

impl FrameContext {
    pub(crate) fn new(encoder: CommandEncoder, texture: SwapchainTexture) -> Self {
        Self { encoder, texture }
    }

    pub(crate) fn finish(self, queue: &Queue) {
        queue.submit(std::iter::once(self.encoder.finish()));
        self.texture.present();
    }
}
