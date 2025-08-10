use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Aabb {
	pub min: Vec3,
	pub max: Vec3,
}
impl Aabb {
	// https://tavianator.com/2015/ray_box_nan.html
	pub fn basic_hit(&self, ray: Ray) -> bool {
		let inv = ray.direction().reciprocal();

		let t1 = (self.min - ray.origin()) * inv;
		let t2 = (self.max - ray.origin()) * inv;

		let tmin = t1.min(t2).horizontal_max();
		let tmax = t1.max(t2).horizontal_min();

		tmax > tmin.max(0.0)
	}
}
