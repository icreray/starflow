use std::sync::Arc;

use winit::window::Window;

use glued::module_impl;
use starflow_util::Size;


pub struct WindowModule {
	pub window: Arc<Window>
}

impl WindowModule {
	pub fn new(window: Window) -> Self {
		let window = Arc::new(window);
		Self { window }
	}

	pub fn with_title(self, title: &str) -> Self {
		self.window.set_title(title);
		self
	}

	pub fn clone_handle(&self) -> Arc<Window> {
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
