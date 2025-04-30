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
pub use winit::*;
#[cfg(feature = "winit")]
mod winit {
	use ::winit::dpi::PhysicalSize;
	use winit::dpi::Pixel;
	use super::Size;

	impl<P: Pixel> From<PhysicalSize<P>> for Size<P> {
		fn from(value: PhysicalSize<P>) -> Self {
			Self::new(value.width, value.height)
		}
	}
}
