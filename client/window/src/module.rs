use std::sync::Arc;

pub use winit::window::Window as WinitWindow;

use glued::module_impl;
use starflow_util::Size;

pub struct WindowModule {
	pub window: Arc<WinitWindow>
}

impl WindowModule {
	pub fn new(window: WinitWindow) -> Self {
		let window = Arc::new(window);
		Self { window }
	}

	pub fn with_title(self, title: &str) -> Self {
		self.window.set_title(title);
		self
	}

	pub fn clone_handle(&self) -> Arc<WinitWindow> {
		self.window.clone()
	}

	pub fn size(&self) -> Size<u32> {
		self.window
			.inner_size()
			.into()
	}
}

#[module_impl(A)]
impl WindowModule {}
