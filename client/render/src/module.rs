use wgpu::{
	RenderPassColorAttachment, RenderPassDescriptor, SurfaceTarget, 
	TextureViewDescriptor
};

use glued::{module::Module, module_impl};
use starflow_util::Size;

use crate::{core::{GpuContext, RenderSurface}, GpuContextConfig};

#[derive(Module)]
pub struct RenderModule<'window> {
	context: GpuContext,
	// Naive approach
	surface: RenderSurface<'window>
	
}

impl<'w> RenderModule<'w> {
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
}

#[module_impl(A)]
impl RenderModule<'_> {
	#[requires(Self)]
	pub fn update(app: &mut A) {
		let renderer = app.module_mut::<Self>();
		let swapchain_texture = renderer.surface
			.get_swapchain_texture(&renderer.context.device)
			.expect("Failed to obtain swapchain texture");

		let mut encoder = renderer.context
			.create_encoder("main_encoder");

		let view = swapchain_texture.texture.create_view(
			&TextureViewDescriptor::default()
		);
		
		encoder.begin_render_pass(&RenderPassDescriptor { 
			label: Some("render_pass"), 
			color_attachments: &[Some(RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations { 
					load: wgpu::LoadOp::Clear(wgpu::Color {
						r: 0.0066, g: 0.0018, b: 0.011, a: 1.0
					}), 
					store: wgpu::StoreOp::Store 
				}
			})],
			depth_stencil_attachment: None,
			timestamp_writes: None, 
			occlusion_query_set: None
		});

		renderer.context.queue.submit(std::iter::once(encoder.finish()));
		swapchain_texture.present();
	}
}
