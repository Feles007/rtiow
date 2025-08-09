use crate::ray::Ray;
use glm::Vec3;

#[derive(Debug)]
pub struct Aabb {
	pub min: Vec3,
	pub max: Vec3,
}
impl Aabb {
	// https://tavianator.com/2015/ray_box_nan.html
	pub fn basic_hit(&self, ray: Ray) -> bool {
		let mut tmin;
		let mut tmax;

		let t1 = (self.min.x - ray.origin().x) / ray.direction().x;
		let t2 = (self.max.x - ray.origin().x) / ray.direction().x;

		tmin = t1.min(t2);
		tmax = t1.max(t2);

		let t1 = (self.min.y - ray.origin().y) / ray.direction().y;
		let t2 = (self.max.y - ray.origin().y) / ray.direction().y;

		tmin = tmin.max(t1.min(t2));
		tmax = tmax.min(t1.max(t2));

		let t1 = (self.min.z - ray.origin().z) / ray.direction().z;
		let t2 = (self.max.z - ray.origin().z) / ray.direction().z;

		tmin = tmin.max(t1.min(t2));
		tmax = tmax.min(t1.max(t2));

		tmax > tmin.max(0.0)
	}
}
