use default::default;

use wgpu::{
	BindGroup, BindGroupDescriptor, Device, Texture, TextureDescriptor, TextureDimension,
	TextureFormat, TextureUsages, TextureView
};

use starflow_util::Size;
use crate::{
	core::{util::AsBindGroupEntry, RenderSurface},
	resources::{BindGroupLayouts, Pipelines}
};


pub(crate) struct RenderResources {
	#[allow(dead_code)]
	bind_group_layouts: BindGroupLayouts,
	pub pipelines: Pipelines,
	pub allocated: AllocatedResources
}

impl RenderResources {
	//TODO: Initialize from Assets
	pub fn new(device: &Device, surface: &RenderSurface) -> Self {
		let bind_group_layouts = BindGroupLayouts::new(device);
		let pipelines = Pipelines::new(
			device,
			&bind_group_layouts,
			surface.texture_format()
		);
		let allocated = AllocatedResources::new(
			device, 
			&bind_group_layouts, 
			surface.size()
		);
		Self { bind_group_layouts, pipelines, allocated }
	}
}


// TODO: Refactoring :D
#[allow(dead_code)]
pub(crate) struct AllocatedResources {
	output_texture: Texture,
	output_texture_view: TextureView,
	pub output_texture_bind_group: BindGroup,
	pub input_texture_bind_group: BindGroup
}

impl AllocatedResources {
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
			layout: &layouts.output_texture,
			entries: &[
				output_texture_view.as_bind_group_entry(0)
			]
		});
		let input_texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
			label: Some("input_texture_bind_group"),
			layout: &layouts.input_texture,
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
