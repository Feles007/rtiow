use crate::bvh::bounding_volume::{BoundingVolume, BoundingVolumeInner};
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
		split::split(&mut nodes, &mut spheres, 0);
		split::skip_pass(&mut nodes, 0, None);
		Self {
			materials,
			nodes,
			spheres,
		}
	}
}
impl MaterialStore for BvhWorld {
	fn get_material(&self, material: MaterialReference) -> &Material {
		&self.materials[usize::from(material.id())]
	}
}
impl Hittable for BvhWorld {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
		let mut current_index = 0;
		let mut hit_record: Option<HitRecord> = None;

		loop {
			let current_node = &self.nodes[current_index];
			if current_node.aabb.basic_hit(ray) {
				match &current_node.inner {
					BoundingVolumeInner::Container { sphere_indices } => {
						let result = (&self.spheres[sphere_indices.clone()]).hit(ray, interval);
						match result {
							Some(result) if hit_record.is_none() || (hit_record.as_ref().unwrap().t > result.t) => {
								hit_record = Some(result);
							},
							_ => {},
						}
						match current_node.skip {
							None => break,
							Some(i) => current_index = i,
						}
					},
					BoundingVolumeInner::Split { left, .. } => {
						current_index = *left;
					},
				}
			} else {
				match current_node.skip {
					None => break,
					Some(i) => current_index = i,
				}
			}
		}

		hit_record
	}
}
