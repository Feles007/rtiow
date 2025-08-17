use crate::bvh::aabb::Aabb;
use crate::bvh::object_range::ObjectRange;
use crate::sphere::Sphere;
use fml::Vec3;

#[derive(Debug)]
pub struct BoundingVolume {
	pub aabb: Aabb,
	pub skip: Option<usize>,
	pub inner: BoundingVolumeInner,
}
#[derive(Debug)]
pub enum BoundingVolumeInner {
	Split { left: usize, right: usize },
	Container { sphere_indices: ObjectRange },
}
impl BoundingVolume {
	pub fn from_spheres(spheres: &[Sphere], range: ObjectRange) -> Self {
		let mut min = Vec3::ZERO;
		let mut max = Vec3::ZERO;

		let mut iter = spheres[range.indices()].iter();

		match iter.next() {
			Some(sphere) => {
				min = sphere.center - Vec3::splat(sphere.radius);
				max = sphere.center + Vec3::splat(sphere.radius);

				for sphere in iter {
					let limit_min = sphere.center - Vec3::splat(sphere.radius);
					let limit_max = sphere.center + Vec3::splat(sphere.radius);

					min = min.min(limit_min);
					max = max.max(limit_max);
				}
			},
			None => {},
		}

		Self {
			aabb: Aabb { min, max },
			skip: None,
			inner: BoundingVolumeInner::Container { sphere_indices: range },
		}
	}
}
