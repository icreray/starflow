use default::default;

use glued::module_impl;

use crate::{
	assets::{RenderAssets, RenderAssetsCreation},
	core::{util::SizedSurfaceTarget, FrameContext, GpuContext, RenderSurface},
	graph::RenderGraph,
	resources::RenderResources,
	GpuContextConfig
};


pub struct Renderer<'window> {
	context: GpuContext,
	surface: RenderSurface<'window>,
	assets: RenderAssets,
	resources: RenderResources,
	graph: RenderGraph
}

impl<'w> Renderer<'w> {
	pub async fn new(
		config: GpuContextConfig<'_>,
		surface_target: impl Into<SizedSurfaceTarget<'w>>
	) -> Self {
		let context = GpuContext::new(config).await;

		let target: SizedSurfaceTarget = surface_target.into();
		let surface = RenderSurface::configured(
			target.target, target.size, &context
		).expect("Failed to create surface");

		let assets = RenderAssets::default();
		let resources = RenderResources::new(
			&context.device,
			&assets,
			surface.size()
		);
		let graph = RenderGraph::new(&assets);

		Self {
			context,
			surface,
			assets,
			resources,
			graph
		}
	}

	pub fn create_assets<F>(&mut self, f: F)
	where F: FnOnce(&mut RenderAssetsCreation) {
		let mut ctx = RenderAssetsCreation::new(
			&mut self.assets,
			&self.surface,
			&self.context.device
		);
		f(&mut ctx);
	}

	fn draw_frame(&self) {
		let encoder = self.context.create_encoder("main_encoder");
		let swapchain_texture = self.surface
			.get_swapchain_texture(&self.context.device)
			.expect("Failed to obtain texture");
		let mut frame = FrameContext::new(
			encoder,
			swapchain_texture
		);
		self.graph.run(
			&mut frame,
			&self.assets,
			&self.resources
		);
		frame.finish(&self.context.queue);
	}
}


#[module_impl(A)]
#[dependencies(Self)]
impl Renderer<'_> {
	#[inline(always)]
	pub fn update(app: &mut A) {
		app.module::<Self>().draw_frame();
	}
}
