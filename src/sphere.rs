use crate::material::MaterialReference;
use crate::utils::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
	pub center: Point3,
	pub radius: f32,
	pub material: MaterialReference,
}
impl Sphere {
	pub fn new(center: Point3, radius: f32, material: MaterialReference) -> Self {
		assert!(radius >= 0.0);
		Self {
			center,
			radius,
			material,
		}
	}
}
