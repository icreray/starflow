use wgpu::{
	Color, RenderPassDescriptor, SurfaceTarget
};

use glued::module_impl;
use starflow_util::Size;

use crate::{core::{FrameContext, GpuContext, RenderSurface}, GpuContextConfig};

pub struct Renderer<'window> {
	context: GpuContext,
	// Naive approach
	surface: RenderSurface<'window>
	
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

		Self { context, surface }
	}

	fn draw_frame<F>(&self, f: F)
	where F: FnOnce(&mut FrameContext) {
		let encoder = self.context.create_encoder("main_encoder");
		let swapchain_texture = self.surface
			.get_swapchain_texture(&self.context.device)
			.expect("Failed to obtain texture");
		let mut frame = FrameContext::new(
			encoder, 
			&self.context.queue, 
			swapchain_texture
		);
		f(&mut frame);
		frame.finish();
	}
}

#[module_impl(A)]
#[dependencies(Self)]
impl Renderer<'_> {
	pub fn update(app: &mut A) {
		app.module::<Self>().draw_frame(|frame| {
			let attachment = frame.texture
				.clear_attachment(Color { r: 0.0066, g: 0.0018, b: 0.011, a: 1.0 });
			frame.encoder.begin_render_pass(&RenderPassDescriptor { 
				label: Some("clear_pass"),
				color_attachments: &[Some(attachment)],
				depth_stencil_attachment: None,
				timestamp_writes: None, 
				occlusion_query_set: None 
			});
		});
	}
}
