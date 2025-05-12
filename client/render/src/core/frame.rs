use wgpu::{CommandEncoder, Queue};

use super::SwapchainTexture;

pub(crate) struct FrameContext<'frame> {
	pub encoder: CommandEncoder,
	pub queue: &'frame Queue,
	pub texture: SwapchainTexture
}

impl<'f> FrameContext<'f> {
	pub fn new(
		encoder: CommandEncoder, 
		queue: &'f Queue, 
		texture: SwapchainTexture
	) -> Self {
		Self { encoder, queue, texture }
	}

	pub fn finish(self) {
		self.queue.submit(std::iter::once(self.encoder.finish()));
		self.texture.present();
	}
}
