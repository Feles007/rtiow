use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::material::MaterialReference;
use crate::ray::Ray;
use crate::utils::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
	pub center: Point3,
	pub radius: f32,
	pub material: MaterialReference,
}
impl Sphere {
	pub fn new(
		center: Point3,
		radius: f32,
		material: MaterialReference,
	) -> Self {
		assert!(radius >= 0.0);
		Self {
			center,
			radius,
			material,
		}
	}
	pub fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
		let oc = self.center - ray.origin();
		let a = ray.direction().magnitude_squared();
		let h = ray.direction().dot(&oc);
		let c = oc.magnitude_squared() - self.radius * self.radius;

		let discriminant = h * h - a * c;
		if discriminant < 0.0 {
			return None;
		}

		let sqrtd = discriminant.sqrt();

		let mut root = (h - sqrtd) / a;
		if root <= interval.min || interval.max <= root {
			root = (h + sqrtd) / a;
			if root <= interval.min || interval.max <= root {
				return None;
			}
		}

		let point = ray.at(root);
		let normal = (point - self.center) / self.radius;

		Some(HitRecord::new(ray, normal, point, root, self.material))
	}
}
