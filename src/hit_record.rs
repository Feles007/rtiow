use crate::material::MaterialReference;
use crate::ray::Ray;
use crate::utils::Point3;
use crate::vec3::Vec3;

pub struct HitRecord {
	pub point: Point3,
	pub normal: Vec3,
	pub t: f32,
	pub front_face: bool,
	pub material: MaterialReference,
}
impl HitRecord {
	pub fn new(ray: Ray, outward_normal: Vec3, point: Point3, t: f32, material: MaterialReference) -> Self {
		let front_face = ray.direction().dot(outward_normal) < 0.0;
		let normal = if front_face { outward_normal } else { -outward_normal };
		Self {
			point,
			normal,
			t,
			front_face,
			material,
		}
	}
}
