use wgpu::{Surface, SurfaceConfiguration, SurfaceTarget};

use starflow_util::Size;
use crate::core::GpuContext;

pub(crate) struct RenderSurface<'window> {
	surface: Surface<'window>,
	config: SurfaceConfiguration
}

impl<'w> RenderSurface<'w> {
	/// Returns none if surface is not supported by adapter
	pub fn configured(
		target: impl Into<SurfaceTarget<'w>>,
		size: Size<u32>, 
		context: &GpuContext
	) -> Option<Self> {
		let surface = context.instance
			.create_surface(target)
			.expect("Failed to create surface");

		let config = surface.get_default_config(
			&context.adapter, 
			size.width, 
			size.height
		)?;

		surface.configure(&context.device, &config);
		Some(Self { surface, config })
	}
}
