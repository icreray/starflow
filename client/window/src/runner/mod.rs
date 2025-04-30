mod handler;

use glued::{module::With, ModularApp};
use handler::AppHandler;
use log::error;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::module::WindowModule;

pub struct WinitRunner;
impl WinitRunner {
	pub fn run<A>(app: A)
	where A: ModularApp + With<WindowModule> {
		// FIXME replace `except` with proper error handling
		let event_loop = EventLoop::new()
			.expect("Failed to create event loop");

		event_loop.set_control_flow(ControlFlow::Poll);

		let mut handler = AppHandler::new(app);
		if let Err(err) = event_loop.run_app(&mut handler) {
			error!("winit event_loop exited with an error: {}", err);
		}
	}
}
