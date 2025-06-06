use default::default;
use log::error;

use winit::{
	application::ApplicationHandler, event::WindowEvent,
	event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
	window::{Window, WindowId}
};

use glued::{AppRunner, ModularApp};


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


pub(super) struct AppHandler<A>
where A: ModularApp {
	app: Option<A>
}

impl<A> Default for AppHandler<A>
where A: ModularApp {
	fn default() -> Self {
		Self { app: Default::default() }
	}
}

impl<A> ApplicationHandler for AppHandler<A>
where A: ModularApp + From<Window> {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		if self.app.is_none() {
			let window = event_loop.create_window(default())
				.expect("Failed to create window");
			self.app = Some(A::from(window));
			self.app_mut().setup();
		}
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		match event {
			WindowEvent::CloseRequested => event_loop.exit(),
			_ => {}
		}
	}

	fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
		self.app_mut().update();
	}
}

impl<A> AppHandler<A>
where A: ModularApp {
	/// # Safety
	/// App should be [`Option::Some`] after [`ApplicationHandler::resumed`] is called
	#[must_use]
	#[inline(always)]
	fn app_mut(&mut self) -> &mut A {
		unsafe { self.app.as_mut().unwrap_unchecked() }
	}
}
