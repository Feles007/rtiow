use crate::hittable::{Hittable, MaterialStore};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rng;
use crate::state::make_look;
use crate::utils::{linear_to_gamma, Color};
use crate::vec3::Vec3;
use bytemuck::{Pod, Zeroable};
use rayon::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Pixel {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}
pub struct Camera {
	pub samples_per_pixel: u32,
	pub max_depth: u32,
	pub fov: f32,
	pub location: Vec3,
	pub pitch: f32,
	pub yaw: f32,
}
impl Camera {
	pub fn render(
		&self,
		world: &(impl Hittable + MaterialStore + Sync),
		buffer: &mut [Pixel],
		width: u32,
		height: u32,
	) {
		let camera_center = self.location;
		let direction = make_look(self.pitch, self.yaw);

		let focal_length = direction.magnitude();
		let theta = self.fov.to_radians();
		let h = (theta / 2.0).tan();
		let viewport_height = 2.0 * h * focal_length;
		let viewport_width = viewport_height * (width as f32 / height as f32);

		let up_vector = Vec3::new(0.0, 1.0, 0.0);

		let w = direction.normalize();
		let u = up_vector.cross(w).normalize();
		let v = w.cross(u);

		let viewport_u = viewport_width * u;
		let viewport_v = viewport_height * -v;

		let pixel_delta_u = viewport_u / (width as f32);
		let pixel_delta_v = viewport_v / (height as f32);

		let viewport_upper_left = camera_center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
		let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

		buffer.par_iter_mut().enumerate().for_each(|(i, pixel)| {
			let y = i as u32 / width;
			let x = i as u32 % width;

			let mut color = Color::ZERO;
			for _ in 0..self.samples_per_pixel {
				let ray = get_ray(x, y, camera_center, pixel00_loc, pixel_delta_u, pixel_delta_v);
				color += ray_color_simple(ray, world, self.max_depth);
			}
			color /= self.samples_per_pixel as f32;

			color = linear_to_gamma(color);

			const INTERVAL: Interval = Interval::new(0.0, 0.999);

			let rgb = [
				(256.0 * INTERVAL.clamp(color.x())) as u8,
				(256.0 * INTERVAL.clamp(color.y())) as u8,
				(256.0 * INTERVAL.clamp(color.z())) as u8,
			];

			pixel.r = rgb[0];
			pixel.g = rgb[1];
			pixel.b = rgb[2];
			pixel.a = 255;
		});
	}
}
fn get_ray(x: u32, y: u32, camera_center: Vec3, pixel00_loc: Vec3, pixel_delta_u: Vec3, pixel_delta_v: Vec3) -> Ray {
	let offset = Vec3::new(rng::f32() - 0.5, rng::f32() - 0.5, 0.0);
	let pixel_sample =
		pixel00_loc + ((x as f32 + offset.x()) * pixel_delta_u) + ((y as f32 + offset.y()) * pixel_delta_v);
	let ray_origin = camera_center;
	let ray_direction = pixel_sample - camera_center;
	Ray::new(ray_origin, ray_direction)
}
#[allow(unused)]
fn ray_color(ray: Ray, world: &(impl Hittable + MaterialStore), depth: u32) -> Color {
	if depth == 0 {
		return Color::ZERO;
	}
	if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
		let mat = world.get_material(hit_record.material);
		return if let Some((scattered, attenuation)) = mat.scatter(ray, hit_record) {
			attenuation * ray_color(scattered, world, depth - 1)
		} else {
			Color::ZERO
		};
	}

	background_color(ray)
}
#[allow(unused)]
fn ray_color_iterative(ray: Ray, world: &(impl Hittable + MaterialStore), depth: u32) -> Color {
	let mut colors = Vec::new();

	let mut current_ray = ray;
	for _ in 0..depth {
		if let Some(hit_record) = world.hit(current_ray, Interval::new(0.001, f32::INFINITY)) {
			let mat = world.get_material(hit_record.material);
			let (hit_color, next_ray) = if let Some((scattered, attenuation)) = mat.scatter(current_ray, hit_record) {
				(attenuation, Some(scattered))
			} else {
				(Color::new(0.0, 0.0, 0.0), None)
			};
			colors.push(hit_color);
			if let Some(next_ray) = next_ray {
				current_ray = next_ray;
			} else {
				break;
			}
		} else {
			break;
		};
	}

	let mut color_accumulator = background_color(current_ray);

	while let Some(color) = colors.pop() {
		color_accumulator = color_accumulator * color;
	}

	color_accumulator
}
#[allow(unused)]
fn ray_color_simple(ray: Ray, world: &(impl Hittable + MaterialStore), depth: u32) -> Color {
	let mut color = Vec3::ZERO;
	let mut first_color = true;

	let mut current_ray = ray;
	for _ in 0..depth {
		if let Some(hit_record) = world.hit(current_ray, Interval::new(0.001, f32::INFINITY)) {
			let mat = world.get_material(hit_record.material);
			let (hit_color, next_ray) = if let Some((scattered, attenuation)) = mat.scatter(current_ray, hit_record) {
				(attenuation, Some(scattered))
			} else {
				(Color::new(0.0, 0.0, 0.0), None)
			};

			if first_color {
				color = hit_color;
				first_color = false;
			} else {
				color = color * hit_color;
			}

			if let Some(next_ray) = next_ray {
				current_ray = next_ray;
			} else {
				break;
			}
		} else {
			break;
		};
	}

	let bgc = background_color(current_ray);

	if first_color { bgc } else { color * bgc }
}
fn background_color(ray: Ray) -> Color {
	let unit_direction = ray.direction().normalize();
	let a = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - a) * Vec3::ONE + a * Vec3::new(0.5, 0.7, 1.0)
}
