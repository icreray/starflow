mod render;

use glued::{AppRunner, ModularApp};
use starflow_render::Renderer;
use starflow_window::{WindowModule, WinitRunner};

use crate::render::create_renderer;


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


pub fn main() {
	env_logger::init();
	WinitRunner::run::<ClientApp>();
}
