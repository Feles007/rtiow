use crate::app::ControlMap;
use crate::bvh::BvhWorld;
use crate::camera::Camera;
use glm::{vec3, Vec3};
use pixels::{Pixels, SurfaceTexture};
use std::f32::consts::FRAC_PI_2;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::{CursorGrabMode, Window};

pub struct State {
	world: BvhWorld,
	camera: Camera,
	window: Arc<Window>,
	pixels: Pixels<'static>,
	size: PhysicalSize<u32>,
	mouse_focused: bool,
}
impl State {
	pub fn new(world: BvhWorld, window: Arc<Window>) -> Self {
		rayon::ThreadPoolBuilder::new().num_threads(16).build_global().unwrap();

		let size = window.inner_size();
		let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
		let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

		let pitch = 0.0;
		let yaw = FRAC_PI_2;

		let camera = Camera {
			samples_per_pixel: 10,
			max_depth: 10,
			fov: 75.0,
			location: vec3(13.0, 2.0, 3.0),
			pitch,
			yaw,
		};

		Self {
			world,
			camera,
			window,
			pixels,
			size,
			mouse_focused: false,
		}
	}
	pub fn focus(&mut self) {
		let result = self.window.set_cursor_grab(CursorGrabMode::Confined);
		match result {
			Ok(_) => {},
			Err(_) => {
				self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
			},
		}
		self.window.set_cursor_visible(false);
		self.mouse_focused = true;
	}
	pub fn unfocus(&mut self) {
		self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
		self.window.set_cursor_visible(true);
		self.mouse_focused = false;
	}
	pub fn is_mouse_focused(&self) -> bool {
		self.mouse_focused
	}
	pub fn request_redraw(&self) {
		self.window.request_redraw();
	}
	pub fn update(&mut self, control_map: &mut ControlMap, delta_time: f32) {
		let zoom_speed = 10.0;
		let sensitivity = 0.01;
		let movement_speed = 5.0;

		if control_map.zoom_in {
			self.camera.fov -= zoom_speed * delta_time;
		} else if control_map.zoom_out {
			self.camera.fov += zoom_speed * delta_time;
		}

		self.camera.pitch += control_map.move_pitch * sensitivity;
		self.camera.yaw -= control_map.move_yaw * sensitivity;
		control_map.move_pitch = 0.0;
		control_map.move_yaw = 0.0;

		let ys = self.camera.yaw.sin();
		let yc = self.camera.yaw.cos();

		let backward = vec3(ys, 0.0, yc);
		let up = vec3(0.0, 1.0, 0.0);
		let left = backward.cross(&up);

		if control_map.move_forward {
			self.camera.location -= backward * movement_speed * delta_time;
		} else if control_map.move_backward {
			self.camera.location += backward * movement_speed * delta_time;
		}
		if control_map.move_left {
			self.camera.location += left * movement_speed * delta_time;
		} else if control_map.move_right {
			self.camera.location -= left * movement_speed * delta_time;
		}
	}
	pub fn render(&mut self) {
		let frame = self.pixels.frame_mut();
		self.camera.render(
			&self.world,
			bytemuck::cast_slice_mut(frame),
			self.size.width,
			self.size.height,
		);
		self.pixels.render().unwrap();
	}
	pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
		self.size = new_size;
		self.pixels.resize_buffer(new_size.width, new_size.height).unwrap();
		self.pixels.resize_surface(new_size.width, new_size.height).unwrap();
	}
}
pub fn make_look(pitch: f32, yaw: f32) -> Vec3 {
	vec3(yaw.sin() * pitch.cos(), pitch.sin(), yaw.cos() * pitch.cos())
}
