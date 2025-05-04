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

	pub fn create_context(&self) -> WindowContext {
		let size = self.window.inner_size()
			.into();
		let window = self.window.clone();
		WindowContext {window, size}
	}
}

#[module_impl(A)]
impl WindowModule {}

pub struct WindowContext {
	pub window: Arc<WinitWindow>,
	pub size: Size<u32>
}
