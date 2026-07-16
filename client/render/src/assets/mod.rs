pub mod desc;
pub mod util;

use std::ops::Index;

use thiserror::Error;
use wgpu::Device;

use starflow_util::{Handle, HasRegistry, Registry, multiregistry};

use crate::{core::RenderSurface, util::Key};


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
        Self {
            assets,
            surface,
            device
        }
    }

    #[allow(private_bounds)]
    pub fn create<'a, D>(&mut self, descriptor: D) -> AssetResult<'a, Handle<D::Asset>>
    where
        D: RenderAssetDesc<'a>,
        RenderAssets: HasRegistry<Key, D::Asset>
    {
        let key = descriptor.key().into();
        let asset = descriptor.create(self)?;
        Ok(self.assets.get_registry_mut().set(key, asset))
    }
}


pub type AssetResult<'key, R> = Result<R, AssetError<'key>>;

#[derive(Error, Debug)]
pub enum AssetError<'key> {
    #[error("Missing dependecy: {0}")]
    MissingDependency(&'key str)
}


pub trait RenderAssetDesc<'a> {
    type Asset: RenderAsset;

    fn key(&self) -> &str;
    fn create(self, ctx: &RenderAssetsCreation) -> AssetResult<'a, Self::Asset>;
}


pub trait RenderAsset: sealed::RenderAsset {}
impl<T: sealed::RenderAsset> RenderAsset for T {}

mod sealed {
    pub trait RenderAsset {}

    impl RenderAsset for wgpu::BindGroupLayout {}
    impl RenderAsset for wgpu::PipelineLayout {}
    impl RenderAsset for wgpu::ShaderModule {}
    impl RenderAsset for wgpu::RenderPipeline {}
    impl RenderAsset for wgpu::ComputePipeline {}
}


#[derive(Default)]
pub struct RenderAssets {
    bind_group_layouts: Registry<Key, wgpu::BindGroupLayout>,
    pipeline_layouts: Registry<Key, wgpu::PipelineLayout>,
    shader_modules: Registry<Key, wgpu::ShaderModule>,
    render_pipelines: Registry<Key, wgpu::RenderPipeline>,
    compute_pipelines: Registry<Key, wgpu::ComputePipeline>
}

multiregistry! {
    RenderAssets, Key,
    wgpu::BindGroupLayout => bind_group_layouts,
    wgpu::PipelineLayout => pipeline_layouts,
    wgpu::ShaderModule => shader_modules,
    wgpu::RenderPipeline => render_pipelines,
    wgpu::ComputePipeline => compute_pipelines
}

impl RenderAssets {
    #[inline(always)]
    pub fn get_handle<R>(&self, key: &str) -> Option<Handle<R>>
    where Self: HasRegistry<Key, R> {
        self.get_registry().get_handle(key)
    }

    #[inline(always)]
    pub fn get_asset<R>(&self, key: &str) -> Option<&R>
    where Self: HasRegistry<Key, R> {
        self.get_registry().get(key)
    }

    #[inline(always)]
    pub fn get_dependency_handle<'a, R>(
        &self,
        key: &'a str
    ) -> AssetResult<'a, Handle<R>>
    where
        Self: HasRegistry<Key, R>
    {
        self.get_handle(key)
            .ok_or(AssetError::MissingDependency(key))
    }

    #[inline(always)]
    pub fn get_dependency_asset<'a, R>(&self, key: &'a str) -> AssetResult<'a, &R>
    where Self: HasRegistry<Key, R> {
        self.get_asset(key)
            .ok_or(AssetError::MissingDependency(key))
    }
}

impl<R> Index<&Handle<R>> for RenderAssets
where RenderAssets: HasRegistry<Key, R>
{
    type Output = R;

    fn index(&self, index: &Handle<R>) -> &Self::Output { &self.get_registry()[index] }
}
