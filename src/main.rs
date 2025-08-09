extern crate nalgebra_glm as glm;
mod app;
mod bvh;
mod camera;
mod hit_record;
mod hittable;
mod interval;
mod material;
mod ray;
mod rng;
mod sphere;
mod state;
mod utils;
mod world;

use crate::app::App;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::world::World;
use glm::vec3;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Poll);
	let mut app = App::new();
	event_loop.run_app(&mut app).unwrap();
}

fn make_world() -> World {
	let mut world = World::new();

	let ground_material = world.add_material(Material::Lambertian {
		albedo: vec3(0.5, 0.5, 0.5),
	});
	world.add_sphere(Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, ground_material));

	for a in -11..11 {
		for b in -11..11 {
			let a = a as f32;
			let b = b as f32;

			let choose_mat = rng::f32();
			let center = vec3(a + 0.9 * rng::f32(), 0.2, b + 0.9 * rng::f32());

			if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
				let sphere_material;

				if choose_mat < 0.8 {
					// diffuse
					let albedo = rng::vector().component_mul(&rng::vector());
					sphere_material = world.add_material(Material::Lambertian { albedo });
					world.add_sphere(Sphere::new(center, 0.2, sphere_material));
				} else if choose_mat < 0.95 {
					// metal
					let albedo = rng::vector().component_mul(&rng::vector());
					let fuzz = rng::f32_range(0.0, 0.5);
					sphere_material = world.add_material(Material::Metal { albedo, fuzz });
					world.add_sphere(Sphere::new(center, 0.2, sphere_material));
				} else {
					// glass
					sphere_material = world.add_material(Material::Dielectric { refraction_index: 1.5 });
					world.add_sphere(Sphere::new(center, 0.2, sphere_material));
				}
			}
		}
	}

	let material1 = world.add_material(Material::Dielectric { refraction_index: 1.5 });
	world.add_sphere(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1));

	let material2 = world.add_material(Material::Lambertian {
		albedo: vec3(0.4, 0.2, 0.1),
	});
	world.add_sphere(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2));

	let material3 = world.add_material(Material::Metal {
		albedo: vec3(0.7, 0.6, 0.5),

		fuzz: 0.0,
	});
	world.add_sphere(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3));

	// let w = 800;
	// let h = 600;
	// let camera = Camera::new(w, h, 1, 1, 20.0, vec3(13.0, 2.0, 3.0), vec3(0.0, 0.0, 0.0));
	// let start = Instant::now();
	// camera.render(&world);
	// let elapsed = start.elapsed();
	// println!("Render took {:?}", elapsed);
	// let average = elapsed / (w * h);
	// println!("Avg per pixel: {:?}", average);
	world
}
