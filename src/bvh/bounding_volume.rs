use crate::bvh::aabb::Aabb;
use crate::sphere::Sphere;
use glm::Vec3;
use std::ops::Range;

#[derive(Debug)]
pub struct BoundingVolume {
	pub aabb: Aabb,
	pub skip: Option<usize>,
	pub inner: BoundingVolumeInner,
}
#[derive(Debug)]
pub enum BoundingVolumeInner {
	Split { left: usize, right: usize },
	Container { sphere_indices: Range<usize> },
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
			skip: None,
			inner: BoundingVolumeInner::Container { sphere_indices },
		}
	}
}
