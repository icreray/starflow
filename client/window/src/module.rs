use std::sync::Arc;

use glued::{module::Module, module_impl};

use winit::window::Window;

#[derive(Module, Default)]
pub struct WindowModule {
	pub window: Option<Arc<Window>>
}

#[module_impl(A)]
impl WindowModule {}
