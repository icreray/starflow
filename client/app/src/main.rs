use futures_lite::future;

use glued::{AppRunner, ModularApp};
use starflow_render::{GpuContextConfig, Renderer, Features};
use starflow_window::{WindowModule, WinitRunner};


#[derive(ModularApp)]
struct ClientApp<'window>(
	WindowModule,
	Renderer<'window>
);

// FIXME: Better way to initialize app
impl From<WindowModule> for ClientApp<'_> {
	fn from(window: WindowModule) -> Self {
		let window = window.with_title("Starflow");
		let renderer = create_renderer(&window);
		Self (window, renderer)
	}
}

fn create_renderer<'w>(window: &WindowModule) -> Renderer<'w> {
	future::block_on(
		Renderer::new(
			GpuContextConfig::default()
				.add_features(Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES),
			window.clone_handle()
		)
	)
}


pub fn main() {
	env_logger::init();
	WinitRunner::run::<ClientApp>();
}
