use futures_lite::future;

use glued::ModularApp;
use starflow_render::{GpuContextConfig, RenderModule};
use starflow_window::{WinitWindow, FromWindow, WindowModule, WinitRunner};

#[derive(ModularApp)]
struct ClientApp<'window>(
	WindowModule,
	RenderModule<'window>
);

// FIXME: Better way to initialize app
impl<'w> FromWindow for ClientApp<'w> {
	fn from(window: WinitWindow) -> Self {
		let window_module = WindowModule::new(window)
			.with_title("Starflow");
		let context = window_module.create_context();
		let reder_module = future::block_on(
			RenderModule::new(
				GpuContextConfig::default(), 
				context.window, 
				context.size
			)
		);
		Self (window_module, reder_module)
	}
}

pub fn main() {
	env_logger::init();
	let render_module = future::block_on(
		RenderModule::new(default())
	);
	let app = ClientApp(
		WindowModule::default(),
		render_module
	);
	WinitRunner::run(app);
}
