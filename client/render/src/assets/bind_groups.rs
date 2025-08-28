use std::ops::Deref;

use wgpu::{BindGroupLayout, Device};

use crate::assets::{descriptors, resource_cache::{ResourceCache, ResourceId}};


#[derive(Default)]
pub(crate) struct BindGroupLayouts {
	inner: ResourceCache<BindGroupLayout>
}

pub(crate) type BindGroupLayoutId = ResourceId<BindGroupLayout>;

impl BindGroupLayouts {
	pub fn create(
		&mut self,
		device: &Device,
		layout: descriptors::BindGroupLayout
	) -> Option<BindGroupLayoutId> {
		if self.inner.contains_key(layout.key) {
			None
		}
		else {
			let key = layout.key;
			let layout = device.create_bind_group_layout(&layout.into());
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
