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


pub(crate) use binding_types::*;
mod binding_types {
	use wgpu::{BindingType, StorageTextureAccess, TextureFormat, TextureViewDimension};

	pub fn texture_storage_2d(format: TextureFormat, access: StorageTextureAccess) -> BindingType {
		BindingType::StorageTexture { access, format, view_dimension: TextureViewDimension::D2 }
	}
}
