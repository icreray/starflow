use wgpu::{Device, ShaderStages, StorageTextureAccess, TextureFormat};

use crate::{
	assets::{descriptors::BindGroupLayout, BindGroupLayouts, Pipelines},
	core::{util::bind_group_layout::binding, RenderSurface}
};


pub mod descriptors {
	use wgpu::{BindGroupLayoutEntry, BindGroupLayoutDescriptor};


	pub struct BindGroupLayout<'a> {
		pub key: &'a str,
		pub entries: &'a [BindGroupLayoutEntry]
	}

	impl<'a> BindGroupLayout<'a> {
		pub fn new(
			key: &'a str,
			entries: &'a [BindGroupLayoutEntry]
		) -> Self {
			Self { key, entries }
		}
	}

	impl<'a> From<BindGroupLayout<'a>> for BindGroupLayoutDescriptor<'a> {
		fn from(value: BindGroupLayout<'a>) -> Self {
			Self {
				label: Some(value.key),
				entries: value.entries
			}
		}
	}
}


pub(crate) struct RenderAssets {
	pub bind_group_layouts: BindGroupLayouts,
	pub pipelines: Pipelines
}

impl RenderAssets {
	pub fn new(device: &Device, surface: &RenderSurface) -> Self {
		let bind_group_layouts = create_bind_group_layouts(device);
		let pipelines = Pipelines::new(
			device,
			&bind_group_layouts,
			surface.texture_format()
		);
		Self { bind_group_layouts, pipelines }
	}
}


// TODO: Move this outside renderer
pub(crate) fn create_bind_group_layouts(device: &Device) -> BindGroupLayouts {
	let mut layouts = BindGroupLayouts::default();
	layouts.create(device, BindGroupLayout::new("output_texture", &[
			binding(0)
				.visibility(ShaderStages::COMPUTE)
				.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly)
	]));
	layouts.create(device, BindGroupLayout::new("input_texture", &[
			binding(0)
				.visibility(ShaderStages::FRAGMENT)
				.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadOnly)
	]));
	layouts
}
