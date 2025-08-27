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
		device: &Device,
		key: &'static str,
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
	use wgpu::{BindGroupLayoutDescriptor, Device, StorageTextureAccess, TextureFormat};

	use crate::{core::util::bind_group_layout::binding, resources::BindGroupLayouts};


	// TODO: Move this outside renderer
	pub(crate) fn create_bind_group_layouts(device: &Device) -> BindGroupLayouts {
		let mut layouts = BindGroupLayouts::default();
		layouts.add(device, "output_texture", &BindGroupLayoutDescriptor {
			label: Some("output_texture"),
			entries: &[
				binding(0)
					.compute()
					.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly)
			]
		});
		layouts.add(device, "input_texture", &BindGroupLayoutDescriptor {
			label: Some("input_texture"),
			entries: &[
				binding(0)
					.fragment()
					.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadOnly)
			]
		});
		layouts
	}
}
