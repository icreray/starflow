use wgpu::SurfaceTarget;

use glued::{module::Module, module_impl};
use starflow_window::WindowModule;
use starflow_util::Size;

use crate::{core::{GpuContext, RenderSurface}, GpuContextConfig};

#[derive(Module)]
pub struct RenderModule<'window> {
	context: GpuContext,
	// Naive approach
	surface: Option<RenderSurface<'window>>
	
}

impl<'w> RenderModule<'w> {
	pub async fn new(config: GpuContextConfig<'_>) -> Self {
		Self {
			context: GpuContext::new(config).await,
			surface: None
		}
	}

	pub fn insert_surface(
		&mut self,
		target: impl Into<SurfaceTarget<'w>>, 
		size: Size<u32>
	) {
		self.surface = RenderSurface::configured(
			target, size, &self.context
		);
	}
}

#[module_impl(A)]
impl RenderModule<'_> {
	#[requires(Self, WindowModule)]
	pub fn setup(app: &mut A) {
		let window = app.module::<WindowModule>()
			.window
			.as_ref()
			.expect("Window is unitialized on setup stage")
			.clone();
		let size = Size::from(window.inner_size());
		app.module_mut::<Self>()
			.insert_surface(window, size);
	}

	#[requires(Self)]
	pub fn update(app: &mut A) {
		let renderer = app.module_mut::<Self>();
		let swapchain_texture = renderer.surface
			.as_mut()
			.unwrap()
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
						r: 0.1, g: 0.1, b: 0.2, a: 1.0
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
