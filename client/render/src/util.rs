use starflow_util::Size;

use wgpu::{
	BindGroupEntry, BindingResource, Buffer, BufferBinding, Sampler, 
	SurfaceTarget, TextureView
};


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


pub(crate) trait AsBindGroupEntry {
	fn as_bind_group_entry<'a>(&'a self, binding: u32) -> BindGroupEntry<'a>; 
}

impl AsBindGroupEntry for Buffer {
	fn as_bind_group_entry<'a>(&'a self, binding: u32) -> BindGroupEntry<'a> {
		BindGroupEntry { binding, resource: BindingResource::Buffer(BufferBinding {
			buffer: self,
			offset: 0,
			size: None,
		})}
	}
}

impl AsBindGroupEntry for TextureView {
	fn as_bind_group_entry<'a>(&'a self, binding: u32) -> BindGroupEntry<'a> {
		BindGroupEntry { binding, resource: BindingResource::TextureView(self)}
	}
}

impl AsBindGroupEntry for Sampler {
	fn as_bind_group_entry<'a>(&'a self, binding: u32) -> BindGroupEntry<'a> {
		BindGroupEntry { binding, resource: BindingResource::Sampler(self)}
	}
}
