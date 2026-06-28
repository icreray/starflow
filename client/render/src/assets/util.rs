use std::num::NonZero;

pub use wgpu::{TextureFormat, StorageTextureAccess, ShaderStages};
use wgpu::{BindGroupLayoutEntry, BindingType, TextureViewDimension};

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

	pub fn visibility(mut self, visibility: ShaderStages) -> Self {
		self.visibility = visibility;
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
