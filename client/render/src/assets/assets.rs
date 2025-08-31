use core::{error, fmt};
use std::ops::Index;
use default::default;

use wgpu::Device;

use starflow_util::{Handle, Registry};

use crate::core::RenderSurface;


pub struct RenderAssetsCreation<'renderer> {
	pub(super) assets: &'renderer mut RenderAssets,
	pub(super) surface: &'renderer RenderSurface<'renderer>,
	pub(super) device: &'renderer Device
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
	pub fn create<'a, D>(&mut self, descriptor: D) -> Result<Handle<D::Asset>, RenderAssetError<'a>>
	where 
		D: RenderAssetDesc<'a>,
		RenderAssets: HasRegistry<D::Asset>
	{
		let key = descriptor.key().into();
		let asset = descriptor.create(self)?;
		Ok(self.assets.get_registry_mut().set(key, asset))
	}
}


#[derive(Debug)]
pub enum RenderAssetError<'a> {
	MissingDependency(&'a str)
}

impl<'a> fmt::Display for RenderAssetError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::MissingDependency(dep) => {write!(f, "Missing dependency {}", dep)}
		}
	}
}

impl<'a> error::Error for RenderAssetError<'a> {}


pub trait RenderAssetDesc<'a> {
	type Asset: sealed::RenderAsset;

	fn key(&self) -> &str;
	fn create(self, ctx: &RenderAssetsCreation) -> Result<Self::Asset, RenderAssetError<'a>>;
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
	render_pipelines: RenderPipelines,
	compute_pipelines: ComputePipelines
}

impl RenderAssets {
	#[allow(private_bounds)]
	pub fn get_handle<R>(&self, key: &str) -> Option<Handle<R>>
	where
		R: sealed::RenderAsset,
		Self: HasRegistry<R>
	{
		self.get_registry().get_handle(key)
	}

	#[allow(private_bounds)]
	pub fn get_asset<R>(&self, key: &str) -> Option<&R>
	where
		R: sealed::RenderAsset,
		Self: HasRegistry<R>
	{
		self.get_registry().get(key)
	}
}

impl<R> Index<&Handle<R>> for RenderAssets
where
	R: sealed::RenderAsset,
	RenderAssets: HasRegistry<R>
{
	type Output = R;

	fn index(&self, index: &Handle<R>) -> &Self::Output {
		&self.get_registry()[index]
	}
}


trait HasRegistry<A>
where A: sealed::RenderAsset {
	fn get_registry(&self) -> &Registry<Box<str>, A>;
	fn get_registry_mut(&mut self) -> &mut Registry<Box<str>, A>;
}

macro_rules! impl_has_registry {
	($render_assets:ty, $asset_ty:ty, $field:ident) => {
		impl HasRegistry<$asset_ty> for $render_assets {
			fn get_registry(&self) -> &Registry<Box<str>, $asset_ty> {
				&self.$field
			}

			fn get_registry_mut(&mut self) -> &mut Registry<Box<str>, $asset_ty> {
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


// TODO: Move this outside renderer with error handling
pub(crate) fn create_render_assets(surface: &RenderSurface, device: &Device) -> RenderAssets {
	use wgpu::{ShaderStages, TextureFormat, StorageTextureAccess};
	use crate::core::util::bind_group_layout::binding;
	use super::desc::*;

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
		)).unwrap();
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
