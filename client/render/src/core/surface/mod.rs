use wgpu::{
	Device, Surface, SurfaceConfiguration, SurfaceError, 
	SurfaceTarget, SurfaceTexture
};

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
			.create_surface(target).ok()?;

		let config = surface.get_default_config(
			&context.adapter,
			// wgpu will panic if one of dimensions is zero
			size.width.max(1), 
			size.height.max(1)
		)?;

		surface.configure(&context.device, &config);
		Some(Self { surface, config })
	}

	pub fn get_swapchain_texture(&self, device: &Device) -> Result<SurfaceTexture, SurfaceError> {
		let texture = match self.surface.get_current_texture() {
			Ok(texture) => texture,
			Err(SurfaceError::Outdated) => {
				self.reconfigure(device);
				self.surface.get_current_texture()?
			}
			Err(e) => return Err(e)
		};
		Ok(texture)
	}

	#[allow(dead_code)]
	pub fn resize(&mut self, size: Size<u32>, device: &Device) {
		self.config.width = size.width.max(1);
		self.config.height = size.height.max(1);
		self.reconfigure(device);
	}

	#[inline(always)]
	fn reconfigure(&self, device: &Device) {
		self.surface.configure(device, &self.config);
	}
}
