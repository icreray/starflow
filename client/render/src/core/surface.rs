use wgpu::{
    Color,
    CurrentSurfaceTexture::{self, *},
    Device, LoadOp, RenderPassColorAttachment, StoreOp, Surface, SurfaceConfiguration,
    SurfaceTarget, SurfaceTexture, TextureFormat, TextureView
};

use starflow_util::{Size, default};

use crate::core::GpuContext;


pub(crate) struct RenderSurface<'window> {
    surface: Surface<'window>,
    config: SurfaceConfiguration
}

impl<'w> RenderSurface<'w> {
    /// Returns none if surface is not supported by adapter
    pub fn configured(
        target: impl Into<SurfaceTarget<'w>>,
        size: Size<u32>,
        context: &GpuContext
    ) -> Option<Self> {
        let surface = context.instance.create_surface(target).ok()?;

        let config = surface.get_default_config(
            &context.adapter,
            // wgpu will panic if one of dimensions is zero
            size.width.max(1),
            size.height.max(1)
        )?;

        surface.configure(&context.device, &config);
        Some(Self { surface, config })
    }

    pub fn texture_format(&self) -> TextureFormat { self.config.format }

    pub fn size(&self) -> Size<u32> { Size::new(self.config.width, self.config.height) }

    pub fn get_swapchain_texture(
        &self,
        device: &Device
    ) -> Result<SwapchainTexture, SurfaceError> {
        let texture = match self.surface.get_current_texture() {
            Success(texture) | Suboptimal(texture) => texture,
            Outdated => {
                self.reconfigure(device);
                match self.surface.get_current_texture() {
                    Success(texture) | Suboptimal(texture) => texture,
                    variant => return Err(variant.into())
                }
            }
            variant => return Err(variant.into())
        };
        let view = texture.texture.create_view(&default());
        Ok(SwapchainTexture { texture, view })
    }

    #[allow(dead_code)]
    pub fn resize(&mut self, size: Size<u32>, device: &Device) {
        self.config.width = size.width.max(1);
        self.config.height = size.height.max(1);
        self.reconfigure(device);
    }

    #[inline(always)]
    fn reconfigure(&self, device: &Device) {
        self.surface.configure(device, &self.config);
    }
}

#[derive(Debug)]
pub enum SurfaceError {
    Timeout,
    Occluded,
    Outdated,
    Lost,
    Validation,
    Unknown
}

impl From<CurrentSurfaceTexture> for SurfaceError {
    fn from(value: CurrentSurfaceTexture) -> Self {
        match value {
            Timeout => Self::Timeout,
            Occluded => Self::Occluded,
            Outdated => Self::Outdated,
            Lost => Self::Lost,
            Validation => Self::Validation,
            _ => Self::Unknown
        }
    }
}


pub(crate) struct SwapchainTexture {
    texture: SurfaceTexture,
    view: TextureView
}

impl SwapchainTexture {
    #[inline]
    pub fn width(&self) -> u32 { self.texture.texture.width() }

    #[inline]
    pub fn height(&self) -> u32 { self.texture.texture.height() }

    pub fn clear_attachment(&self, color: Color) -> RenderPassColorAttachment<'_> {
        RenderPassColorAttachment {
            view: &self.view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: LoadOp::Clear(color),
                store: StoreOp::Store
            }
        }
    }

    pub fn present(self) { self.texture.present(); }
}
