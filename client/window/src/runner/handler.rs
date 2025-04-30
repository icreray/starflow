use std::sync::Arc;

use glued::{module::With, ModularApp};
use winit::{
	application::ApplicationHandler, event::WindowEvent, 
	event_loop::ActiveEventLoop, window::{Window, WindowAttributes, WindowId}
};

use crate::module::WindowModule;

pub(super) struct AppHandler<A>
where A: ModularApp {
	app: A
}

impl<A> AppHandler<A>
where A: ModularApp {
	pub fn new(app: A) -> Self {
		Self { app }
	}
}

impl<A> ApplicationHandler for AppHandler<A>
where A: ModularApp + With<WindowModule> {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		self.app.module_mut::<WindowModule>()
			.window
			.get_or_insert_with(|| create_window(event_loop));
		self.app.setup();
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
		self.app.update();
	}
}

fn create_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
	let window = event_loop.create_window(
		WindowAttributes::default()
			.with_title("Starflow")
	).expect("Failed to create window");
	Arc::new(window)
}
