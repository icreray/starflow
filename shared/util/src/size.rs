pub struct Size<T> {
	pub width: T,
	pub height: T
}

impl<T> Size<T> {
	pub fn new(width: T, height: T) -> Self {
		Self { width, height }
	}
}

#[cfg(feature = "winit")]
#[allow(unused_imports)]
pub use winit_features::*;
#[cfg(feature = "winit")]
mod winit_features {
	use winit::dpi::{PhysicalSize, Pixel};
	use super::Size;

	impl<P: Pixel> From<PhysicalSize<P>> for Size<P> {
		fn from(value: PhysicalSize<P>) -> Self {
			Self::new(value.width, value.height)
		}
	}
}

#[cfg(feature = "wgpu")]
#[allow(unused_imports)]
pub use wgpu_features::*;
#[cfg(feature = "wgpu")]
mod wgpu_features {
	use wgpu::Extent3d;
	use super::Size;

	impl From<Size<u32>> for Extent3d {
		fn from(value: Size<u32>) -> Self {
			Self {
				width: value.width,
				height: value.height,
				depth_or_array_layers: 1
			}
		}
	}
}
