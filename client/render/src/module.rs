use wgpu::SurfaceTarget;

use glued::module_impl;
use starflow_util::Size;

use crate::{core::{FrameContext, GpuContext, RenderSurface}, graph::RenderGraph, scene::Scene, GpuContextConfig};

pub struct Renderer<'window> {
	context: GpuContext,
	// Naive approach
	surface: RenderSurface<'window>,
	scene: Scene
}

impl<'w> Renderer<'w> {
	pub async fn new(
		config: GpuContextConfig<'_>,
		surface_target: impl Into<SurfaceTarget<'w>>,
		surface_size: Size<u32>
	) -> Self {
		let context = GpuContext::new(config).await;
		let surface = RenderSurface::configured(
			surface_target, surface_size, &context
		).expect("Failed to create surface");

		Self { context, surface, scene: Scene {} }
	}

	fn draw_frame(&self) {
		let encoder = self.context.create_encoder("main_encoder");
		let swapchain_texture = self.surface
			.get_swapchain_texture(&self.context.device)
			.expect("Failed to obtain texture");
		let mut frame = FrameContext::new(
			encoder, 
			&self.context.queue, 
			swapchain_texture
		);
		RenderGraph::run(&mut frame, &self.scene);
		frame.finish();
	}
}

#[module_impl(A)]
#[dependencies(Self)]
impl Renderer<'_> {
	pub fn update(app: &mut A) {
		app.module::<Self>().draw_frame();
	}
}
