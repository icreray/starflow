use wgpu::{Device, StorageTextureAccess, TextureFormat};

use crate::{
	core::{util::bind_group_layout::binding, RenderSurface}, 
	assets::{BindGroupLayouts, Pipelines}
};


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
