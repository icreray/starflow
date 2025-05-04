use default::default;

use winit::{
	application::ApplicationHandler, event::WindowEvent, 
	event_loop::ActiveEventLoop, window::{Window, WindowId}
};

use glued::ModularApp;

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
	/// ### Safety
	/// App should be [`Option::Some`] after [`ApplicationHandler::resumed`] is called
	#[must_use]
	#[inline(always)]
	fn app_mut(&mut self) -> &mut A {
		unsafe { self.app.as_mut().unwrap_unchecked() }
	}
}
