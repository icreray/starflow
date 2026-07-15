pub mod assets;
pub mod config;
pub mod graph;
pub mod resources;
pub mod util;

mod core;

use starflow_util::default;
use thiserror::Error;

use glued::module_impl;

use crate::{
    assets::{RenderAssets, RenderAssetsCreation},
    config::RendererConfig,
    core::{GpuContext, GpuContextError, RenderSurface, util::SizedSurfaceTarget},
    graph::{FrameContext, RenderGraph, RenderGraphCreation},
    resources::RenderResources
};


pub struct Renderer<'window> {
    context: GpuContext,
    surface: RenderSurface<'window>,
    assets: RenderAssets,
    resources: RenderResources,
    graph: RenderGraph
}

impl<'w> Renderer<'w> {
    pub async fn try_new(
        config: RendererConfig<'_>,
        surface_target: impl Into<SizedSurfaceTarget<'w>>
    ) -> Result<Self, RendererError> {
        let context = GpuContext::try_new(config).await?;

        let target: SizedSurfaceTarget = surface_target.into();
        let surface = RenderSurface::configured(target.target, target.size, &context)
            .ok_or(RendererError::FailedToCreateSurface)?;

        let assets = RenderAssets::default();
        let resources = RenderResources::new(&context.device, &assets, surface.size());

        Ok(Self {
            context,
            surface,
            assets,
            resources,
            graph: default()
        })
    }

    pub fn create_assets<F>(&mut self, f: F)
    where F: FnOnce(&mut RenderAssetsCreation) {
        let mut ctx = RenderAssetsCreation::new(
            &mut self.assets,
            &self.surface,
            &self.context.device
        );
        f(&mut ctx);
    }

    pub fn set_graph<F>(&mut self, f: F)
    where F: FnOnce(&mut RenderGraphCreation) {
        let mut ctx = RenderGraphCreation::new(&mut self.graph, &self.assets);
        f(&mut ctx);
    }

    fn draw_frame(&self) {
        let encoder = self.context.create_encoder("main_encoder");
        let swapchain_texture = self
            .surface
            .get_swapchain_texture(&self.context.device)
            .expect("Failed to obtain texture");
        let mut frame = FrameContext::new(encoder, swapchain_texture);
        self.graph.run(&mut frame, &self.assets, &self.resources);
        frame.finish(&self.context.queue);
    }
}


#[module_impl(A)]
#[dependencies(Self)]
impl Renderer<'_> {
    #[inline(always)]
    pub fn update(app: &mut A) { app.module::<Self>().draw_frame(); }
}


#[derive(Error, Debug)]
pub enum RendererError {
    #[error(transparent)]
    FailedToInitContext(#[from] GpuContextError),
    #[error("Failed to create surface")]
    FailedToCreateSurface
}
