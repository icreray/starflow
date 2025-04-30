use default::default;
use futures_lite::future;

use glued::ModularApp;
use starflow_render::RenderModule;
use starflow_window::{WindowModule, WinitRunner};

#[derive(ModularApp)]
struct ClientApp<'window>(
	WindowModule,
	RenderModule<'window>
);

pub fn main() {
	let render_module = future::block_on(
		RenderModule::new(default())
	);
	let app = ClientApp(
		WindowModule::default(),
		render_module
	);
	WinitRunner::run(app);
}
