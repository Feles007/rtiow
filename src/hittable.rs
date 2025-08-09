use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::material::{Material, MaterialReference};
use crate::ray::Ray;

pub trait MaterialStore {
	fn get_material(&self, material: MaterialReference) -> &Material;
}
pub trait Hittable {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord>;
}
