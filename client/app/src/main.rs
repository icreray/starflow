use futures_lite::future;

use glued::{AppRunner, ModularApp};
use starflow_render::{GpuContextConfig, Renderer};
use starflow_window::{WinitWindow, WindowModule, WinitRunner};

#[derive(ModularApp)]
struct ClientApp<'window>(
	WindowModule,
	Renderer<'window>
);

// FIXME: Better way to initialize app
impl From<WinitWindow> for ClientApp<'_> {
	fn from(window: WinitWindow) -> Self {
		let window_module = WindowModule::new(window)
			.with_title("Starflow");
		let renderer = future::block_on(
			Renderer::new(
				GpuContextConfig::default(),
				window_module.clone_handle(),
				window_module.size()
			)
		);
		Self (window_module, renderer)
	}
}

pub fn main() {
	env_logger::init();
	WinitRunner::run::<ClientApp>();
}
