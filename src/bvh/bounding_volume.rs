use crate::bvh::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use glm::Vec3;
use std::ops::Range;

#[derive(Debug)]
pub struct BoundingVolume {
	pub aabb: Aabb,
	pub inner: BoundingVolumeInner,
}
#[derive(Debug)]
pub enum BoundingVolumeInner {
	Split(usize, usize),
	Container(Range<usize>),
}
impl BoundingVolume {
	pub fn from_spheres(spheres: &[Sphere], sphere_indices: Range<usize>) -> Self {
		let mut min = Vec3::zeros();
		let mut max = Vec3::zeros();

		let mut iter = spheres[sphere_indices.clone()].iter();

		match iter.next() {
			Some(sphere) => {
				min = sphere.center - Vec3::from_element(sphere.radius);
				max = sphere.center + Vec3::from_element(sphere.radius);

				for sphere in iter {
					let limit_min = sphere.center - Vec3::from_element(sphere.radius);
					let limit_max = sphere.center + Vec3::from_element(sphere.radius);

					min.x = min.x.min(limit_min.x);
					min.y = min.y.min(limit_min.y);
					min.z = min.z.min(limit_min.z);

					max.x = max.x.max(limit_max.x);
					max.y = max.y.max(limit_max.y);
					max.z = max.z.max(limit_max.z);
				}
			},
			None => {},
		}

		Self {
			aabb: Aabb { min, max },
			inner: BoundingVolumeInner::Container(sphere_indices),
		}
	}
	pub fn hit(&self, ray: Ray, interval: Interval, spheres: &[Sphere], nodes: &[BoundingVolume]) -> Option<HitRecord> {
		if !self.aabb.basic_hit(ray) {
			return None;
		}

		match &self.inner {
			BoundingVolumeInner::Split(n0, n1) => {
				let hr0 = nodes[*n0].hit(ray, interval, spheres, nodes);
				let hr1 = nodes[*n1].hit(ray, interval, spheres, nodes);

				match (hr0, hr1) {
					(None, None) => None,
					(Some(hr), None) => Some(hr),
					(None, Some(hr)) => Some(hr),
					(Some(hr0), Some(hr1)) => Some(if hr0.t < hr1.t { hr0 } else { hr1 }),
				}
			},
			BoundingVolumeInner::Container(sphere_indices) => (&spheres[sphere_indices.clone()]).hit(ray, interval),
		}
	}
}
