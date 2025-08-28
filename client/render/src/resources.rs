use default::default;

use wgpu::{
	BindGroup, BindGroupDescriptor, Device, Texture, TextureDescriptor, TextureDimension,
	TextureFormat, TextureUsages, TextureView
};

use starflow_util::Size;

use crate::{
	core::util::AsBindGroupEntry,
	assets::BindGroupLayouts
};


// TODO: Refactoring :D
#[allow(dead_code)]
pub(crate) struct RenderResources {
	output_texture: Texture,
	output_texture_view: TextureView,
	pub output_texture_bind_group: BindGroup,
	pub input_texture_bind_group: BindGroup
}

impl RenderResources {
	pub fn new(
		device: &Device,
		layouts: &BindGroupLayouts,
		surface_size: Size<u32>
	) -> Self {
		let output_texture = device.create_texture(&TextureDescriptor {
			label: Some("output_texture"),
			size: surface_size.into(),
			mip_level_count: 1,
			sample_count: 1,
			dimension: TextureDimension::D2,
			format: TextureFormat::Rgba8Unorm,
			usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
			view_formats: &[]
		});
		let output_texture_view = output_texture.create_view(&default());
		let output_texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("output_texture_bind_group"),
			layout: &layouts.get("output_texture").unwrap(),
			entries: &[
				output_texture_view.as_bind_group_entry(0)
			]
		});
		let input_texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("input_texture_bind_group"),
			layout: &layouts.get("input_texture").unwrap(),
			entries: &[
				output_texture_view.as_bind_group_entry(0)
			]
		});
		Self {
			output_texture,
			output_texture_view,
			output_texture_bind_group,
			input_texture_bind_group
		}
	}
}
