use crate::bvh::bounding_volume::BoundingVolume;
use crate::bvh::split;
use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, MaterialStore};
use crate::interval::Interval;
use crate::material::{Material, MaterialReference};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::world::World;

#[derive(Debug)]
pub struct BvhWorld {
	materials: Vec<Material>,
	nodes: Vec<BoundingVolume>,
	spheres: Vec<Sphere>,
}
impl BvhWorld {
	pub fn new(world: World) -> Self {
		let (mut spheres, materials) = world.decompose();
		let mut nodes = vec![BoundingVolume::from_spheres(&spheres, 0..spheres.len())];
		split::split(&mut nodes, &mut spheres);
		Self {
			materials,
			nodes,
			spheres,
		}
	}
}
impl Hittable for BvhWorld {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
		self.nodes[0].hit(ray, interval, &self.spheres)
	}
}
impl MaterialStore for BvhWorld {
	fn get_material(&self, material: MaterialReference) -> &Material {
		&self.materials[usize::from(material.id())]
	}
}
