pub mod desc;
pub mod util;

use std::ops::Index;

use wgpu::Device;

use starflow_util::{Handle, HasRegistry, Registry, multiregistry};

use crate::{
    core::{RenderObjectDesc, RenderObjectError, RenderObjectResult, RenderSurface},
    util::Key
};


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

    pub fn create<'desc, D>(
        &mut self,
        descriptor: D
    ) -> RenderObjectResult<'desc, Handle<D::Object>>
    where
        D: RenderObjectDesc<'desc, Self>,
        RenderAssets: HasRegistry<Key, D::Object>
    {
        let key = descriptor.key().into();
        let asset = descriptor.try_create(self)?;
        Ok(self.assets.get_registry_mut().set(key, asset))
    }
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
    pub fn get_dependency_handle<'key, R>(
        &self,
        key: &'key str
    ) -> RenderObjectResult<'key, Handle<R>>
    where
        Self: HasRegistry<Key, R>
    {
        self.get_handle(key)
            .ok_or(RenderObjectError::MissingDependency(key))
    }

    #[inline(always)]
    pub fn get_dependency_asset<'key, R>(
        &self,
        key: &'key str
    ) -> RenderObjectResult<'key, &R>
    where
        Self: HasRegistry<Key, R>
    {
        self.get_asset(key)
            .ok_or(RenderObjectError::MissingDependency(key))
    }
}

impl<R> Index<&Handle<R>> for RenderAssets
where RenderAssets: HasRegistry<Key, R>
{
    type Output = R;

    fn index(&self, index: &Handle<R>) -> &Self::Output { &self.get_registry()[index] }
}
