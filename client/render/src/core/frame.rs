use wgpu::{CommandEncoder, Queue};

use super::SwapchainTexture;


pub(crate) struct FrameContext {
	pub encoder: CommandEncoder,
	pub texture: SwapchainTexture
}

impl FrameContext {
	pub fn new(
		encoder: CommandEncoder,
		texture: SwapchainTexture
	) -> Self {
		Self { encoder, texture }
	}

	pub fn finish(self, queue: &Queue) {
		queue.submit(std::iter::once(self.encoder.finish()));
		self.texture.present();
	}
}
