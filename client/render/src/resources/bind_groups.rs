use wgpu::{BindGroupLayout, Device};


/* TODO:
/ Same asset management as described in pipelines.rs
*/
pub(crate) struct BindGroupLayouts {
	pub output_texture: BindGroupLayout,
	pub input_texture: BindGroupLayout
}

impl BindGroupLayouts {
	pub fn new(device: &Device) -> Self {
		Self {
			output_texture: assets::create_output_texture_bind_group_layout(device),
			input_texture: assets::create_input_texture_bind_group_layout(device)
		}
	}
}


mod assets {
	use wgpu::{
		BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device,
		ShaderStages, StorageTextureAccess, TextureFormat, TextureViewDimension
	};

	pub(super) fn create_output_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
		device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: Some("output_texture"),
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::COMPUTE,
					ty: BindingType::StorageTexture {
						access: StorageTextureAccess::WriteOnly,
						format: TextureFormat::Rgba8Unorm,
						view_dimension: TextureViewDimension::D2
					},
					count: None
				}
			]
		})
	}

	pub(super) fn create_input_texture_bind_group_layout(device: &Device) -> BindGroupLayout {
		device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: Some("input_texture"),
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::StorageTexture {
						access: StorageTextureAccess::ReadOnly,
						format: TextureFormat::Rgba8Unorm,
						view_dimension: TextureViewDimension::D2
					},
					count: None
				}
			]
		})
	}
}
