use starflow_window::WindowModule;
use wgpu::SurfaceTarget;

use glued::{module::Module, module_impl};
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
		// TODO: run render graph
	}
}
