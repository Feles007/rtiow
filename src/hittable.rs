use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::ray::Ray;

pub trait Hittable {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord>;
}
