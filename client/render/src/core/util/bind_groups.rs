use wgpu::{BindGroupEntry, BindingResource, Buffer, BufferBinding, Sampler, TextureView};


pub mod bind_group_layout {
	use std::num::NonZero;

	pub use wgpu::{TextureFormat, StorageTextureAccess};
	use wgpu::{BindGroupLayoutEntry, BindingType, ShaderStages, TextureViewDimension};


	pub struct BindGroupLayoutEntryBuilder {
		binding: u32,
		visibility: ShaderStages,
		count: Option<NonZero<u32>>
	}

	pub fn binding(binding: u32) -> BindGroupLayoutEntryBuilder {
		BindGroupLayoutEntryBuilder { binding, visibility: ShaderStages::NONE, count: None }
	}

	#[allow(dead_code)]
	impl BindGroupLayoutEntryBuilder {
		pub fn vertex(mut self) -> Self {
			self.visibility |= ShaderStages::VERTEX;
			self
		}

		pub fn fragment(mut self) -> Self {
			self.visibility |= ShaderStages::FRAGMENT;
			self
		}

		pub fn compute(mut self) -> Self {
			self.visibility |= ShaderStages::COMPUTE;
			self
		}

		pub fn count(mut self, count: NonZero<u32>) -> Self {
			self.count = Some(count);
			self
		}

		pub fn texture_storage_2d(
			self,
			format: TextureFormat,
			access: StorageTextureAccess
		) -> BindGroupLayoutEntry {
			BindGroupLayoutEntry {
				binding: self.binding,
				visibility: self.visibility, 
				ty: BindingType::StorageTexture { access, format, view_dimension: TextureViewDimension::D2 },
				count: self.count
			}
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
