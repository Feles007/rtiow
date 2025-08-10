use crate::bvh::aabb::Aabb;
use crate::bvh::object_range::ObjectRange;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

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

					min = Vec3::new(
						min.x().min(limit_min.x()),
						min.y().min(limit_min.y()),
						min.z().min(limit_min.z()),
					);

					max = Vec3::new(
						max.x().max(limit_max.x()),
						max.y().max(limit_max.y()),
						max.z().max(limit_max.z()),
					);
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
