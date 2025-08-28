use std::ops::Deref;

use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, Device};

use crate::assets::resource_cache::{ResourceCache, ResourceId};


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
