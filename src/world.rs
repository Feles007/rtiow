use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::{Material, MaterialReference};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct World {
	spheres: Vec<Sphere>,
	materials: Vec<Material>,
}
impl World {
	pub fn new() -> Self {
		Self {
			spheres: Vec::new(),
			materials: Vec::new(),
		}
	}
	pub fn add_material(&mut self, material: Material) -> MaterialReference {
		let id = MaterialReference::new(self.materials.len());
		self.materials.push(material);
		id
	}
	pub fn get_material(&self, material: MaterialReference) -> &Material {
		&self.materials[usize::from(material.id())]
	}
	pub fn add_sphere(&mut self, sphere: Sphere) {
		self.spheres.push(sphere);
	}
}
impl Hittable for World {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
		let mut closest_so_far = interval.max;
		let mut ret = None;

		for sphere in self.spheres.iter() {
			if let Some(result) = sphere.hit(ray, Interval::new(interval.min, closest_so_far)) {
				closest_so_far = result.t;
				ret = Some(result);
			}
		}

		ret
	}
}
