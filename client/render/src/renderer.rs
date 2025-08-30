use glued::module_impl;

use crate::{
	assets::{create_render_assets, HasRegistry, RenderAssets},
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

		let assets = create_render_assets(&surface, &context.device);
		let resources = RenderResources::new(
			&context.device,
			&assets.get(),
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
