use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, MaterialStore};
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
	pub fn add_sphere(&mut self, sphere: Sphere) {
		self.spheres.push(sphere);
	}
	pub fn decompose(self) -> (Vec<Sphere>, Vec<Material>) {
		(self.spheres, self.materials)
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
impl MaterialStore for World {
	fn get_material(&self, material: MaterialReference) -> &Material {
		&self.materials[usize::from(material.id())]
	}
}
