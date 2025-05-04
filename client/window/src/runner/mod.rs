mod handler;

use log::error;
use winit::{event_loop::{ControlFlow, EventLoop}, window::Window};

use glued::{AppRunner, ModularApp};

use crate::runner::handler::AppHandler;

pub struct WinitRunner;
impl AppRunner for WinitRunner {
	type Context = Window;

	fn run<A>()
	where A: ModularApp + From<Self::Context> {
		// FIXME replace `except` with proper error handling
		let event_loop = EventLoop::new()
			.expect("Failed to create event loop");

		event_loop.set_control_flow(ControlFlow::Poll);

		let mut handler = AppHandler::<A>::default();
		if let Err(err) = event_loop.run_app(&mut handler) {
			error!("winit event_loop exited with an error: {}", err);
		}
	}
}
