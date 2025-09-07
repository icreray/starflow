use default::default;

use wgpu::{Color, ComputePassDescriptor, ComputePipeline, RenderPassDescriptor, RenderPipeline};

use starflow_util::Handle;

use crate::{
	assets::{AssetError, AssetResult, RenderAssets},
	core::FrameContext,
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
	where R: 'static
	{
		self.nodes.push(Box::new(node))
	}
}


#[allow(private_interfaces)]
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
	pub(crate) fn new(
		graph: &'r mut RenderGraph,
		assets: &'r RenderAssets
	) -> Self {
		Self { graph, assets }
	}

	#[allow(private_bounds)]
	pub fn add_node<R: RenderNode>(&mut self) -> AssetResult<'r, ()>
	where R: TryFrom<&'r RenderAssets, Error = AssetError<'r>> + 'static
	{
		let node: R = self.assets.try_into()?;
		self.graph.add_node(node);
		Ok(())
	}
}


pub struct MainPass {
	main_pass: Handle<ComputePipeline>
}

impl<'a> TryFrom<&'a RenderAssets> for MainPass {
	type Error = AssetError<'a>;
	
	fn try_from(assets: &'a RenderAssets) -> AssetResult<'a, Self> {
		let main_pass = assets.get_dependency_handle("main_pass")?;
		Ok(Self { main_pass })
	}
}

impl RenderNode for MainPass {
	fn run(
		&self,
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		let mut pass = frame.encoder.begin_compute_pass(&ComputePassDescriptor {
			label: Some("main_pass"),
			timestamp_writes: None,
		});
		pass.set_pipeline(&assets[&self.main_pass]);
		pass.set_bind_group(0, &resources.output_texture_bind_group, &[]);
		pass.dispatch_workgroups(
			(frame.texture.width() + 15) >> 4,
			(frame.texture.height() + 15) >> 4,
			1
		);
	}
}


pub struct BlitPass {
	blit: Handle<RenderPipeline>
}

impl<'a> TryFrom<&'a RenderAssets> for BlitPass {
	type Error = AssetError<'a>;
	
	fn try_from(assets: &'a RenderAssets) -> AssetResult<'a, Self> {
		let blit = assets.get_dependency_handle("blit")?;
		Ok(Self { blit })
	}
}

impl RenderNode for BlitPass {
	fn run(
		&self,
		frame: &mut FrameContext,
		assets: &RenderAssets,
		resources: &RenderResources
	) {
		let attachment = frame.texture.clear_attachment(Color::BLACK);
		let mut pass = frame.encoder.begin_render_pass(&RenderPassDescriptor {
			label: Some("display"),
			color_attachments: &[Some(attachment)],
			..default()
		});
		pass.set_pipeline(&assets[&self.blit]);
		pass.set_bind_group(0, &resources.input_texture_bind_group, &[]);
		pass.draw(0..3, 0..1);
	}
}
