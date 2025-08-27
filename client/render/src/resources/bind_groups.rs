use std::ops::Index;
use ahash::AHashMap;

use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, Device};


#[derive(Default)]
pub(crate) struct BindGroupLayouts {
	key_to_id: AHashMap<&'static str, BindGroupLayoutId>,
	layouts: Vec<BindGroupLayout>
}

#[derive(Clone, Copy)]
pub(crate) struct BindGroupLayoutId(usize);

impl BindGroupLayouts {

	// TODO: 'Neat' layout creation
	pub fn add(
		&mut self,
		key: &'static str,
		device: &Device,
		descriptor: &BindGroupLayoutDescriptor
	) -> Option<BindGroupLayoutId> {
		if self.key_to_id.contains_key(key) {
			None
		}
		else {
			let id = BindGroupLayoutId(self.layouts.len());
			self.key_to_id.insert(key, id);
			self.layouts.push(device.create_bind_group_layout(descriptor));
			Some(id)
		}
	}

	pub fn get(&self, key: &str) -> Option<&BindGroupLayout> {
		self.get_id(key).map(|id| &self[id])
	}

	pub fn get_id(&self, key: &str) -> Option<BindGroupLayoutId> {
		self.key_to_id.get(key).cloned()
	}
}

impl Index<BindGroupLayoutId> for BindGroupLayouts {
	type Output = BindGroupLayout;

	fn index(&self, index: BindGroupLayoutId) -> &Self::Output {
		&self.layouts[index.0]
	}
}


pub(crate) mod assets {
	use wgpu::{
		BindGroupLayoutDescriptor, BindGroupLayoutEntry, Device,
		ShaderStages, StorageTextureAccess, TextureFormat
	};

	use crate::{core::util::texture_storage_2d, resources::BindGroupLayouts};

	// TODO: Move this outside renderer
	pub(crate) fn create_bind_group_layouts(device: &Device) -> BindGroupLayouts {
		let mut layouts = BindGroupLayouts::default();
		layouts.add("output_texture", device, &BindGroupLayoutDescriptor {
			label: Some("output_texture"),
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::COMPUTE,
					ty: texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
					count: None
				}
			]
		});
		layouts.add("input_texture", device, &BindGroupLayoutDescriptor {
			label: Some("input_texture"),
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::FRAGMENT,
					ty: texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadOnly),
					count: None
				}
			]
		});
		layouts
	}
}
