use core::{error, fmt};
use std::ops::Index;

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
	pub fn create<'a, D>(&mut self, descriptor: D) -> AssetResult<'a, Handle<D::Asset>>
	where 
		D: RenderAssetDesc<'a>,
		RenderAssets: HasRegistry<D::Asset>
	{
		let key = descriptor.key().into();
		let asset = descriptor.create(self)?;
		Ok(self.assets
			.get_registry_mut()
			.set(key, asset)
		)
	}
}


pub type AssetResult<'a, R> = Result<R, AssetError<'a>>;

#[derive(Debug)]
pub enum AssetError<'a> {
	MissingDependency(&'a str)
}

impl<'a> fmt::Display for AssetError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::MissingDependency(dep) => {write!(f, "Missing dependency {}", dep)}
		}
	}
}

impl<'a> error::Error for AssetError<'a> {}


pub trait RenderAssetDesc<'a> {
	type Asset: sealed::RenderAsset;

	fn key(&self) -> &str;
	fn create(self, ctx: &RenderAssetsCreation) -> AssetResult<'a, Self::Asset>;
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

	#[allow(private_bounds)]
	pub fn get_dependency_handle<'a, R>(&self, key: &'a str) -> AssetResult<'a, Handle<R>>
	where
		R: sealed::RenderAsset,
		Self: HasRegistry<R>
	{
		self.get_handle(key)
			.ok_or(AssetError::MissingDependency(key))
	}

	#[allow(private_bounds)]
	pub fn get_dependency_asset<'a, R>(&self, key: &'a str) -> AssetResult<'a, &R>
	where
		R: sealed::RenderAsset,
		Self: HasRegistry<R>
	{
		self.get_asset(key)
			.ok_or(AssetError::MissingDependency(key))
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
