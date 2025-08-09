use crate::material::Material;
use crate::sphere::Sphere;
use crate::world::World;
use glm::Vec3;

pub struct BvhWorld {
	materials: Vec<Material>,
}
impl BvhWorld {
	pub fn new(world: World) -> Self {
		let (spheres, materials) = world.decompose();
		Self { materials }
	}
}
struct Aabb {
	min: Vec3,
	max: Vec3,
}
impl Aabb {
	pub fn from_spheres(spheres: &[Sphere]) -> Self {
		let mut min = Vec3::zeros();
		let mut max = Vec3::zeros();

		for sphere in spheres {
			let limit_max = sphere.center + Vec3::from_element(sphere.radius);
			let limit_min = sphere.center - Vec3::from_element(sphere.radius);

			min.x = min.x.min(limit_min.x);
			min.y = min.y.min(limit_min.y);
			min.z = min.z.min(limit_min.z);

			max.x = max.x.max(limit_max.x);
			max.y = max.y.max(limit_max.y);
			max.z = max.z.max(limit_max.z);
		}

		Self { min, max }
	}
}
