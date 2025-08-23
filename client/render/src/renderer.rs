use default::default;

use wgpu::SurfaceTarget;

use glued::module_impl;
use starflow_util::Size;

use crate::{
	core::{FrameContext, GpuContext, RenderSurface},
	graph::RenderGraph,
	scene::Scene, GpuContextConfig,
	util::SizedSurfaceTarget
};


pub struct Renderer<'window> {
	context: GpuContext,
	surface: RenderSurface<'window>,
	resources: RenderResources,
	scene: Scene
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

		let resources = RenderResources::new(
			&context.device, surface.texture_format()
		);
		let scene = Scene::new(
			&context.device,
			&resources.bind_group_layouts
		);

		Self {
			context,
			surface,
			resources,
			scene
		}
	}

	fn draw_frame(&self) {
		let encoder = self.context.create_encoder("main_encoder");
		let swapchain_texture = self.surface
			.get_swapchain_texture(&self.context.device)
			.expect("Failed to obtain texture");
		let mut frame = FrameContext::new(
			&self.context.device,
			encoder,
			swapchain_texture
		);
		RenderGraph::run(
			&mut frame,
			&self.scene,
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
