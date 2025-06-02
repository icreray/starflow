use wgpu::{CommandEncoder, Device, Queue};

use super::SwapchainTexture;

pub(crate) struct FrameContext<'frame> {
	pub device: &'frame Device,
	pub encoder: CommandEncoder,
	pub texture: SwapchainTexture
}

impl<'f> FrameContext<'f> {
	pub fn new(
		device: &'f Device,
		encoder: CommandEncoder,
		texture: SwapchainTexture
	) -> Self {
		Self { device, encoder, texture }
	}

	pub fn finish(self, queue: &Queue) {
		queue.submit(std::iter::once(self.encoder.finish()));
		self.texture.present();
	}
}
