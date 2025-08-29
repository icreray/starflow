use std::ops::Deref;

use starflow_util::{Handle, Registry};
use wgpu::{BindGroupLayout, Device};

use crate::assets::descriptors;


#[derive(Default)]
pub(crate) struct BindGroupLayouts {
	registry: Registry<Box<str>, BindGroupLayout>
}

pub(crate) type BindGroupLayoutId = Handle<BindGroupLayout>;

impl BindGroupLayouts {
	pub fn create(
		&mut self,
		device: &Device,
		layout: descriptors::BindGroupLayout
	) -> BindGroupLayoutId {
		let key = layout.key;
		let layout = device.create_bind_group_layout(&layout.into());
		self.registry.set(key.into(), layout)
	}
}

impl Deref for BindGroupLayouts {
	type Target = Registry<Box<str>, BindGroupLayout>;

	fn deref(&self) -> &Self::Target {
		&self.registry
	}
}
