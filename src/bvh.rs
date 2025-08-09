use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, MaterialStore};
use crate::interval::Interval;
use crate::material::{Material, MaterialReference};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::world::World;
use glm::Vec3;
use std::ops::Range;

#[derive(Debug)]
pub struct BvhWorld {
	materials: Vec<Material>,
	bounding_volume: BoundingVolume,
	spheres: Vec<Sphere>,
}
impl BvhWorld {
	pub fn new(world: World) -> Self {
		let (spheres, materials) = world.decompose();
		Self {
			materials,
			bounding_volume: BoundingVolume::from_spheres(&spheres, 0..spheres.len()),
			spheres,
		}
	}
	pub fn split(&mut self) {
		self.bounding_volume.split(&mut self.spheres);
	}
}
impl Hittable for BvhWorld {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
		self.bounding_volume.hit(ray, interval, &self.spheres)
	}
}
impl MaterialStore for BvhWorld {
	fn get_material(&self, material: MaterialReference) -> &Material {
		&self.materials[usize::from(material.id())]
	}
}
#[derive(Debug)]
pub struct Aabb {
	min: Vec3,
	max: Vec3,
}
impl Aabb {
	// https://tavianator.com/2015/ray_box_nan.html
	pub fn basic_hit(&self, ray: Ray) -> bool {
		let mut tmin;
		let mut tmax;

		let t1 = (self.min.x - ray.origin().x) / ray.direction().x;
		let t2 = (self.max.x - ray.origin().x) / ray.direction().x;

		tmin = t1.min(t2);
		tmax = t1.max(t2);

		let t1 = (self.min.y - ray.origin().y) / ray.direction().y;
		let t2 = (self.max.y - ray.origin().y) / ray.direction().y;

		tmin = tmin.max(t1.min(t2));
		tmax = tmax.min(t1.max(t2));

		let t1 = (self.min.z - ray.origin().z) / ray.direction().z;
		let t2 = (self.max.z - ray.origin().z) / ray.direction().z;

		tmin = tmin.max(t1.min(t2));
		tmax = tmax.min(t1.max(t2));

		tmax > tmin.max(0.0)
	}
}
#[derive(Debug)]
pub struct BoundingVolume {
	aabb: Aabb,
	inner: BoundingVolumeInner,
}
#[derive(Debug)]
pub enum BoundingVolumeInner {
	Split(Box<[BoundingVolume; 2]>),
	Container(Range<usize>),
}
enum Axis {
	X,
	Y,
	Z,
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
	pub fn split(&mut self, spheres: &mut [Sphere]) {
		let (c0, c1) = {
			let sphere_indices = match &mut self.inner {
				BoundingVolumeInner::Split(_) => unimplemented!(),
				BoundingVolumeInner::Container(spheres) => spheres.clone(),
			};

			if sphere_indices.len() < 3 {
				return;
			}

			let spheres = &mut spheres[sphere_indices.clone()];

			let axis = {
				let x_span = self.aabb.min.x.abs() + self.aabb.max.x.abs();
				let y_span = self.aabb.min.y.abs() + self.aabb.max.y.abs();
				let z_span = self.aabb.min.z.abs() + self.aabb.max.z.abs();

				if x_span > y_span && x_span > z_span {
					Axis::X
				} else if y_span > x_span && y_span > z_span {
					Axis::Y
				} else {
					Axis::Z
				}
			};

			spheres.sort_by(|a, b| match axis {
				Axis::X => a.center.x.partial_cmp(&b.center.x).unwrap(),
				Axis::Y => a.center.y.partial_cmp(&b.center.y).unwrap(),
				Axis::Z => a.center.z.partial_cmp(&b.center.z).unwrap(),
			});

			let start = sphere_indices.clone().start;
			let end = sphere_indices.clone().end;
			let si_half = start + sphere_indices.len() / 2;

			let r0 = start..si_half;
			let r1 = si_half..end;

			assert_eq!(r0.len() + r1.len(), sphere_indices.len());

			(r0, r1)
		};

		let mut bvi0 = BoundingVolume::from_spheres(spheres, c0);
		bvi0.split(spheres);
		let mut bvi1 = BoundingVolume::from_spheres(spheres, c1);
		bvi1.split(spheres);

		self.inner = BoundingVolumeInner::Split(Box::new([bvi0, bvi1]));
	}
	pub fn hit(&self, ray: Ray, interval: Interval, spheres: &[Sphere]) -> Option<HitRecord> {
		if !self.aabb.basic_hit(ray) {
			return None;
		}

		match &self.inner {
			BoundingVolumeInner::Split(split) => {
				let hr0 = split[0].hit(ray, interval, spheres);
				let hr1 = split[1].hit(ray, interval, spheres);

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
