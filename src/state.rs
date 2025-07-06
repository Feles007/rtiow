use crate::app::ControlMap;
use crate::camera::Camera;
use crate::world::World;
use glm::vec3;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct State {
	world: World,
	camera: Camera,
	window: Arc<Window>,
	pixels: Pixels<'static>,
	size: PhysicalSize<u32>,
}
impl State {
	pub fn new(world: World, window: Arc<Window>) -> Self {
		rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

		let size = window.inner_size();
		let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
		let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

		let camera = Camera {
			samples_per_pixel: 1,
			max_depth: 10,
			fov: 20.0,
			look_from: vec3(13.0, 2.0, 3.0),
			look_at: vec3(0.0, 0.0, 0.0),
		};

		Self {
			world,
			camera,
			window,
			pixels,
			size,
		}
	}
	pub fn request_redraw(&self) {
		self.window.request_redraw();
	}
	pub fn update(&mut self, control_map: &mut ControlMap, delta_time: f32) {
		let zoom_speed = 10.0;

		if control_map.zoom_in {
			self.camera.fov -= zoom_speed * delta_time;
		} else if control_map.zoom_out {
			self.camera.fov += zoom_speed * delta_time;
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
