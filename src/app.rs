use crate::make_world;
use crate::state::State;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

#[derive(Debug, Default, Copy, Clone)]
pub struct ControlMap {
	pub zoom_in: bool,
	pub zoom_out: bool,
}

pub enum App {
	Initializing,
	Running {
		state: State,
		control_map: ControlMap,
		delta_time: f32,
	},
}
impl App {
	pub fn new() -> Self {
		Self::Initializing
	}
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());

		let state = State::new(make_world(), window.clone());

		window.request_redraw();
		*self = Self::Running {
			state,
			control_map: Default::default(),
			delta_time: 0.0,
		};
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
		let Self::Running {
			state,
			control_map,
			delta_time,
		} = self
		else {
			return;
		};
		match event {
			WindowEvent::CloseRequested => {
				event_loop.exit();
			},
			WindowEvent::KeyboardInput { event, .. } => {
				let (code, pressed) = {
					let (key, pressed) = match event {
						KeyEvent {
							physical_key,
							state: ElementState::Pressed,
							..
						} => (physical_key, true),
						KeyEvent {
							physical_key,
							state: ElementState::Released,
							..
						} => (physical_key, false),
					};
					match key {
						PhysicalKey::Code(code) => (code, pressed),
						_ => return,
					}
				};

				*match code {
					KeyCode::KeyZ => &mut control_map.zoom_in,
					KeyCode::KeyX => &mut control_map.zoom_out,

					_ => return,
				} = pressed;
			},
			WindowEvent::RedrawRequested => {
				let start = Instant::now();
				state.update(control_map, *delta_time);
				state.render();
				let elapsed = start.elapsed();

				*delta_time = elapsed.as_secs_f32();

				state.request_redraw();
			},
			WindowEvent::Resized(size) => {
				state.resize(size);
			},
			_ => (),
		}
	}
}
