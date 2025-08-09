use crate::hit_record::HitRecord;
use crate::hittable::{Hittable, MaterialStore};
use crate::interval::Interval;
use crate::material::{Material, MaterialReference};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::world::World;
use glm::Vec3;

#[derive(Debug)]
pub struct BvhWorld {
	materials: Vec<Material>,
	bounding_volume: BoundingVolume,
}
impl BvhWorld {
	pub fn new(world: World) -> Self {
		let (spheres, materials) = world.decompose();
		Self {
			materials,
			bounding_volume: BoundingVolume::from_spheres(spheres),
		}
	}
	pub fn split(&mut self) {
		self.bounding_volume.split();
	}
}
impl Hittable for BvhWorld {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
		self.bounding_volume.hit(ray, interval)
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
	// https://alelievr.github.io/Modern-Rendering-Introduction/AABBIntersection/
	pub fn basic_hit(&self, ray: Ray, interval: Interval) -> bool {
		let p0 = ray.at(interval.min);
		let p1 = ray.at(interval.max);

		let c = (self.min + self.max) * 0.5; // Box center
		let e = self.max - c; // Box half-extent

		// Segment midpoint and halflength vector
		let mut m = (p0 + p1) * 0.5; // Segment midpoint
		let d = p1 - m; // Segment halflength vector
		m = m - c; // Translate box and segment to the origin

		// Test world coordinate axes as separating axes
		let mut adx = d.x.abs();
		if m.x.abs() > e.x + adx {
			return false;
		};
		let mut ady = d.y.abs();
		if m.y.abs() > e.y + ady {
			return false;
		};
		let mut adz = d.z.abs();
		if m.z.abs() > e.z + adz {
			return false;
		};

		// Add a small epsilon to counteract potential arithmetic errors when the segment is
		// near-parallel to one of the coordinate axes
		adx += f32::EPSILON;
		ady += f32::EPSILON;
		adz += f32::EPSILON;

		// Test cross products of segment direction vector with coordinate axes
		if (m.y * d.z - m.z * d.y).abs() > e.y * adz + e.z * ady {
			return false;
		}; // Cross with X-axis
		if (m.z * d.x - m.x * d.z).abs() > e.x * adz + e.z * adx {
			return false;
		}; // Cross with Y-axis
		if (m.x * d.y - m.y * d.x).abs() > e.x * ady + e.y * adx {
			return false;
		}; // Cross with Z-axis

		// No separating axis found; segment overlaps the AABB
		true
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
	Container(Vec<Sphere>),
}
enum Axis {
	X,
	Y,
	Z,
}
impl BoundingVolume {
	pub fn from_spheres(spheres: Vec<Sphere>) -> Self {
		let mut min = Vec3::zeros();
		let mut max = Vec3::zeros();

		let mut iter = spheres.iter();

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
			inner: BoundingVolumeInner::Container(spheres),
		}
	}
	pub fn split(&mut self) {
		let (c0, c1) = {
			let spheres = match &mut self.inner {
				BoundingVolumeInner::Split(_) => unimplemented!(),
				BoundingVolumeInner::Container(spheres) => spheres,
			};

			if spheres.len() < 3 {
				return;
			}

			let axis = {
				let x_span = self.aabb.min.x.abs() + self.aabb.max.x.abs();
				let y_span = self.aabb.min.y.abs() + self.aabb.max.y.abs();
				let z_span = self.aabb.min.z.abs() + self.aabb.max.z.abs();

				if x_span > y_span && x_span > z_span {
					Axis::X
				} else if y_span > x_span && y_span > z_span {
					Axis::Y
				} else if z_span > x_span && z_span > y_span {
					Axis::Z
				} else {
					dbg!(x_span, y_span, z_span);
					todo!()
				}
			};

			spheres.sort_by(|a, b| match axis {
				Axis::X => a.center.x.partial_cmp(&b.center.x).unwrap(),
				Axis::Y => a.center.y.partial_cmp(&b.center.y).unwrap(),
				Axis::Z => a.center.z.partial_cmp(&b.center.z).unwrap(),
			});

			let (c0, c1) = spheres.split_at(spheres.len() / 2);
			(c0.to_owned(), c1.to_owned())
		};

		let mut bvi0 = BoundingVolume::from_spheres(c0);
		bvi0.split();
		let mut bvi1 = BoundingVolume::from_spheres(c1);
		bvi1.split();

		self.inner = BoundingVolumeInner::Split(Box::new([bvi0, bvi1]));
	}
}
impl Hittable for BoundingVolume {
	fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
		if !self.aabb.basic_hit(ray, interval) {
			return None;
		}

		match &self.inner {
			BoundingVolumeInner::Split(split) => {
				let hr0 = split[0].hit(ray, interval);
				let hr1 = split[1].hit(ray, interval);

				match (hr0, hr1) {
					(None, None) => None,
					(Some(hr), None) => Some(hr),
					(None, Some(hr)) => Some(hr),
					(Some(hr0), Some(hr1)) => Some(if hr0.t < hr1.t { hr0 } else { hr1 }),
				}
			},
			BoundingVolumeInner::Container(spheres) => spheres.as_slice().hit(ray, interval),
		}
	}
}
