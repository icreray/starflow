use std::ops::Deref;

use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Device};

use crate::resources::resource_cache::{ResourceCache, ResourceId};


#[derive(Default)]
pub(crate) struct BindGroupLayouts {
	inner: ResourceCache<BindGroupLayout>
}

pub(crate) type BindGroupLayoutId = ResourceId<BindGroupLayout>;

impl BindGroupLayouts {
	pub fn create(
		&mut self,
		device: &Device,
		key: &'static str,
		entries: &[BindGroupLayoutEntry]
	) -> Option<BindGroupLayoutId> {
		if self.inner.contains_key(key) {
			None
		}
		else {
			let descriptor = BindGroupLayoutDescriptor { label: Some(key), entries };
			let layout = device.create_bind_group_layout(&descriptor);
			Some(self.inner.add_unchecked(key, layout))
		}
	}
}

impl Deref for BindGroupLayouts {
	type Target = ResourceCache<BindGroupLayout>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}


pub(crate) mod assets {
	use wgpu::{Device, StorageTextureAccess, TextureFormat};

	use crate::{core::util::bind_group_layout::binding, resources::BindGroupLayouts};


	// TODO: Move this outside renderer
	pub(crate) fn create_bind_group_layouts(device: &Device) -> BindGroupLayouts {
		let mut layouts = BindGroupLayouts::default();
		layouts.create(device, "output_texture", &[
				binding(0)
					.compute()
					.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly)
		]);
		layouts.create(device, "input_texture", &[
				binding(0)
					.fragment()
					.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadOnly)
		]);
		layouts
	}
}
