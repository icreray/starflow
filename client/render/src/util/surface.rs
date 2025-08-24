use wgpu::SurfaceTarget;

use starflow_util::Size;


pub struct SizedSurfaceTarget<'window> {
	pub target: SurfaceTarget<'window>,
	pub size: Size<u32>
}

#[cfg(feature = "winit")]
#[allow(unused_imports)]
pub use winit_features::*;
#[cfg(feature = "winit")]
mod winit_features {
	use std::sync::Arc;
	use winit::window::Window;

	use super::SizedSurfaceTarget;


	impl<'w> From<Arc<Window>> for SizedSurfaceTarget<'w> {
		fn from(value: Arc<Window>) -> Self {
			let size = value.inner_size().into();
			let target = value.into();
			Self { target, size }
		}
	}
}
