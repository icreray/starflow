use wgpu::{BindGroupEntry, BindingResource, Buffer, BufferBinding, Sampler, TextureView};


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
