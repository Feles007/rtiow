use crate::material::Material;
use crate::world::World;

pub struct BvhWorld {
	materials: Vec<Material>,
}
impl BvhWorld {
	pub fn new(world: World) -> Self {
		let (spheres, materials) = world.decompose();
		Self { materials }
	}
}
