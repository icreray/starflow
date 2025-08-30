use default::default;

use wgpu::Device;

use starflow_util::{Handle, Registry};

use crate::core::RenderSurface;


pub struct RenderAssetsCreation<'renderer> {
	assets: &'renderer mut RenderAssets,
	surface: &'renderer RenderSurface<'renderer>,
	device: &'renderer Device
}

impl<'r> RenderAssetsCreation<'r> {
	pub(crate) fn new(
		assets: &'r mut RenderAssets,
		surface: &'r RenderSurface<'r>,
		device: &'r Device
	) -> Self {
		Self { assets, surface, device }
	}

	#[allow(private_bounds)]
	pub fn create<D>(&mut self, descriptor: D) -> Option<Handle<D::Asset>>
	where 
		D: RenderAssetDesc,
		RenderAssets: HasRegistry<D::Asset>
	{
		let key = descriptor.key().into();
		let asset = descriptor.create(self)?;
		Some(self.assets.get_mut().set(key, asset))
	}
}


pub trait RenderAssetDesc {
	type Asset: sealed::RenderAsset;

	fn key(&self) -> &str;
	fn create(self, ctx: &RenderAssetsCreation) -> Option<Self::Asset>;
}

mod sealed {
	pub trait RenderAsset {}

	impl RenderAsset for wgpu::BindGroupLayout {}
	impl RenderAsset for wgpu::PipelineLayout {}
	impl RenderAsset for wgpu::ShaderModule {}
	impl RenderAsset for wgpu::RenderPipeline {}
	impl RenderAsset for wgpu::ComputePipeline {}
}

pub(crate) type BindGroupLayouts = Registry<Box<str>, wgpu::BindGroupLayout>;
pub(crate) type PipelineLayouts = Registry<Box<str>, wgpu::PipelineLayout>;
pub(crate) type ShaderModules = Registry<Box<str>, wgpu::ShaderModule>;
pub(crate) type RenderPipelines = Registry<Box<str>, wgpu::RenderPipeline>;
pub(crate) type ComputePipelines = Registry<Box<str>, wgpu::ComputePipeline>;

#[derive(Default)]
pub(crate) struct RenderAssets {
	bind_group_layouts: BindGroupLayouts,
	pipeline_layouts: PipelineLayouts,
	shader_modules: ShaderModules,
	pub render_pipelines: RenderPipelines,
	pub compute_pipelines: ComputePipelines
}

pub(crate) trait HasRegistry<A>
where A: sealed::RenderAsset {
	fn get(&self) -> &Registry<Box<str>, A>;
	fn get_mut(&mut self) -> &mut Registry<Box<str>, A>;
}

macro_rules! impl_has_registry {
	($render_assets:ty, $asset_ty:ty, $field:ident) => {
		impl HasRegistry<$asset_ty> for $render_assets {
			fn get(&self) -> &Registry<Box<str>, $asset_ty> {
				&self.$field
			}

			fn get_mut(&mut self) -> &mut Registry<Box<str>, $asset_ty> {
				&mut self.$field
			}
		}
	};
}

impl_has_registry!(RenderAssets, wgpu::BindGroupLayout, bind_group_layouts);
impl_has_registry!(RenderAssets, wgpu::PipelineLayout, pipeline_layouts);
impl_has_registry!(RenderAssets, wgpu::ShaderModule, shader_modules);
impl_has_registry!(RenderAssets, wgpu::RenderPipeline, render_pipelines);
impl_has_registry!(RenderAssets, wgpu::ComputePipeline, compute_pipelines);


pub mod desc {
	use default::default;

	pub use wgpu::{PushConstantRange, ShaderSource, PrimitiveState, DepthStencilState, MultisampleState};
	use wgpu::{
		BindGroupLayoutDescriptor, BindGroupLayoutEntry, BlendState, ColorTargetState, ColorWrites,
		ComputePipelineDescriptor, FragmentState, PipelineLayoutDescriptor, RenderPipelineDescriptor,
		 ShaderModuleDescriptor, VertexState
	};

	use super::{RenderAssetDesc, RenderAssetsCreation};


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

	impl<'a> RenderAssetDesc for BindGroupLayout<'a> {
		type Asset = wgpu::BindGroupLayout;

		fn key(&self) -> &str { &self.key }

		fn create(self, ctx: &RenderAssetsCreation) -> Option<Self::Asset> {
			Some(ctx.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				label: Some(self.key),
				entries: self.entries
			}))
		}
	}


	pub struct PipelineLayout<'a> {
		pub key: &'a str,
		pub bind_group_layouts: &'a [&'a str],
		pub push_constant_ranges: &'a [PushConstantRange]
	}

	impl<'a> RenderAssetDesc for PipelineLayout<'a> {
		type Asset = wgpu::PipelineLayout;

		fn key(&self) -> &str { &self.key }

		fn create(self, ctx: &RenderAssetsCreation) -> Option<Self::Asset> {
			let layouts = self.bind_group_layouts.iter()
				.map(|&layout| ctx.assets.bind_group_layouts.get(layout))
				.collect::<Option<Vec<_>>>()?;
			Some(ctx.device.create_pipeline_layout(&PipelineLayoutDescriptor {
				label: Some(self.key),
				bind_group_layouts: &layouts,
				push_constant_ranges: self.push_constant_ranges
			}))
		}
	}


	pub struct ShaderModule<'a> {
		pub key: &'a str,
		pub source: ShaderSource<'a>
	}

	impl<'a> ShaderModule<'a> {
		pub fn new(key: &'a str, source: ShaderSource<'a>) -> Self {
			Self { key, source }
		}
	}

	impl<'a> RenderAssetDesc for ShaderModule<'a> {
		type Asset = wgpu::ShaderModule;
	
		fn key(&self) -> &str { &self.key }
	
		fn create(self, ctx: &RenderAssetsCreation) -> Option<Self::Asset> {
			Some(ctx.device.create_shader_module(ShaderModuleDescriptor {
				label: Some(self.key),
				source: self.source
			}))
		}
	}


	pub struct ComputePipeline<'a> {
		pub key: &'a str,
		pub layout: Option<&'a str>,
		pub module: &'a str
	}

	impl<'a> RenderAssetDesc for ComputePipeline<'a> {
		type Asset = wgpu::ComputePipeline;
	
		fn key(&self) -> &str { &self.key }
	
		fn create(self, ctx: &RenderAssetsCreation) -> Option<Self::Asset> {
			let layout = match self.layout {
				Some(layout) => Some(ctx.assets.pipeline_layouts.get(layout)?),
				None => None
			};
			let module = ctx.assets.shader_modules.get(self.module)?;

			Some(ctx.device.create_compute_pipeline(&ComputePipelineDescriptor {
				label: Some(self.key),
				layout,
				module,
				entry_point: None,
				// TODO: More granular control (if needed, otherwise remove this todo in future :))
				compilation_options: default(),
				cache: None
			}))
		}
	}


	// TODO: This is proof of concept. Add more granular control over shaders and other parameters
	pub struct RenderPipeline<'a> {
		pub key: &'a str,
		pub layout: Option<&'a str>,
		pub vertex: &'a str,
		pub fragment: Option<&'a str>,
		pub primitive: PrimitiveState,
		pub depth_stencil: Option<DepthStencilState>,
		pub multisample: MultisampleState
	}

	impl<'a> RenderAssetDesc for RenderPipeline<'a> {
		type Asset = wgpu::RenderPipeline;
	
		fn key(&self) -> &str { &self.key }
	
		fn create(self, ctx: &RenderAssetsCreation) -> Option<Self::Asset> {
			let layout = match self.layout {
				Some(layout) => Some(ctx.assets.pipeline_layouts.get(layout)?),
				None => None
			};
			let vertex = ctx.assets.shader_modules.get(self.vertex)?;
			let fragment = match self.fragment {
				Some(fragment) => {
					let fragment = ctx.assets.shader_modules.get(fragment)?;
					Some(FragmentState {
						module: fragment,
						entry_point: None,
						compilation_options: default(),
						targets: &[Some(ColorTargetState {
							format: ctx.surface.texture_format(),
							blend: Some(BlendState::REPLACE),
							write_mask: ColorWrites::ALL,
						})]
					})
				}
				None => None
			};
			Some(ctx.device.create_render_pipeline(&RenderPipelineDescriptor {
				label: Some(self.key),
				layout,
				vertex: VertexState {
					module: vertex,
					entry_point: None,
					compilation_options: default(),
					buffers: &[]
				},
				fragment,
				primitive: self.primitive,
				depth_stencil: self.depth_stencil,
				multisample: self.multisample,
				multiview: None,
				cache: None
			}))
		}
	}
}


// TODO: Move this outside renderer with error handling
pub(crate) fn create_render_assets(surface: &RenderSurface, device: &Device) -> RenderAssets {
	use wgpu::{ShaderStages, TextureFormat, StorageTextureAccess};
	use crate::core::util::bind_group_layout::binding;
	use desc::*;

	let mut assets = RenderAssets::default();
	{
		let mut ctx = RenderAssetsCreation::new(&mut assets, surface, device);

		ctx.create(BindGroupLayout::new("output_texture", &[
				binding(0)
					.visibility(ShaderStages::COMPUTE)
					.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly)
		])).unwrap();
		ctx.create(BindGroupLayout::new("input_texture", &[
				binding(0)
					.visibility(ShaderStages::FRAGMENT)
					.texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadOnly)
		])).unwrap();
		// main pass
		ctx.create(PipelineLayout {
			key: "main_pass",
			bind_group_layouts: &["output_texture"],
			push_constant_ranges: &[]
		}).unwrap();
		ctx.create(ShaderModule::new("main_pass",
			ShaderSource::Wgsl(include_str!("../../../../assets/shaders/main_pass.wgsl").into())
		));
		ctx.create(ComputePipeline {
			key: "main_pass",
			layout: Some("main_pass"),
			module: "main_pass",
		}).unwrap();
		// blit
		ctx.create(ShaderModule::new("fullscreen",
			ShaderSource::Wgsl(include_str!("../../../../assets/shaders/fullscreen.wgsl").into())
		)).unwrap();
		ctx.create(ShaderModule::new("blit",
			ShaderSource::Wgsl(include_str!("../../../../assets/shaders/blit.wgsl").into())
		)).unwrap();
		ctx.create(PipelineLayout {
			key: "blit",
			bind_group_layouts: &["input_texture"],
			push_constant_ranges: &[]
		}).unwrap();
		ctx.create(RenderPipeline {
			key: "blit",
			layout: Some("blit"),
			vertex: "fullscreen",
			fragment: Some("blit"),
			primitive: default(),
			depth_stencil: None,
			multisample: default()
		}).unwrap();
	}
	assets
}
