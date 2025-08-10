use crate::bvh::bounding_volume::{BoundingVolume, BoundingVolumeInner};
use crate::sphere::Sphere;

enum Axis {
	X,
	Y,
	Z,
}
pub fn split(nodes: &mut Vec<BoundingVolume>, spheres: &mut [Sphere], index: usize) {
	let (c0, c1) = {
		let bv = &mut nodes[index];
		let sphere_indices = match &mut bv.inner {
			BoundingVolumeInner::Split { .. } => unimplemented!(),
			BoundingVolumeInner::Container { sphere_indices } => sphere_indices.clone(),
		};

		if sphere_indices.len() < 3 {
			return;
		}

		let spheres = &mut spheres[sphere_indices.clone()];

		let axis = {
			let x_span = bv.aabb.min.x.abs() + bv.aabb.max.x.abs();
			let y_span = bv.aabb.min.y.abs() + bv.aabb.max.y.abs();
			let z_span = bv.aabb.min.z.abs() + bv.aabb.max.z.abs();

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

	let n0_index = nodes.len();
	nodes.push(BoundingVolume::from_spheres(spheres, c0));
	split(nodes, spheres, n0_index);

	let n1_index = nodes.len();
	nodes.push(BoundingVolume::from_spheres(spheres, c1));
	split(nodes, spheres, n1_index);

	nodes[n0_index].skip = Some(n1_index);
	nodes[n1_index].skip = nodes[index].skip;

	nodes[index].inner = BoundingVolumeInner::Split {
		left: n0_index,
		right: n1_index,
	}
}
pub fn skip_pass(nodes: &mut [BoundingVolume], index: usize, skip: Option<usize>) {
	match nodes[index].inner {
		BoundingVolumeInner::Container { .. } => nodes[index].skip = skip,
		BoundingVolumeInner::Split { left, right } => {
			nodes[index].skip = skip;
			skip_pass(nodes, left, Some(right));
			skip_pass(nodes, right, skip);
		},
	}
}
