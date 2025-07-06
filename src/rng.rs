use glm::{vec3, Vec3};
use std::sync::atomic::{AtomicU32, Ordering};

// thread_local! {
// 	static RNG: RefCell<Xoshiro128Plus> = RefCell::new(Xoshiro128Plus::seed_from_u64(0));
// }
// pub fn f32() -> f32 {
// 	RNG.with_borrow_mut(|rng| rng.random())
// }
// thread_local! {
// 	static RNG_STATE: Cell<u32> = const { Cell::new(0xE9BE815E) };
// }
// pub fn f32() -> f32 {
// 	const SIGN_EXP: u32 = 0x3F800000;
//
// 	let mut x = RNG_STATE.get();
// 	x ^= x << 13;
// 	x ^= x >> 17;
// 	x ^= x << 5;
// 	RNG_STATE.replace(x);
// 	f32::from_bits((x >> 9) | SIGN_EXP) - 1.0
// }
static RNG_STATE: AtomicU32 = AtomicU32::new(0xE9BE815E);

pub fn f32() -> f32 {
	const SIGN_EXP: u32 = 0x3F800000;

	let mut x = RNG_STATE.load(Ordering::Relaxed);
	x ^= x << 13;
	x ^= x >> 17;
	x ^= x << 5;
	RNG_STATE.store(x, Ordering::Relaxed);
	f32::from_bits((x >> 9) | SIGN_EXP) - 1.0
}

pub fn f32_range(min: f32, max: f32) -> f32 {
	min + (max - min) * f32()
}
pub fn vector_range(min: f32, max: f32) -> Vec3 {
	vec3(f32_range(min, max), f32_range(min, max), f32_range(min, max))
}
pub fn unit_vector() -> Vec3 {
	loop {
		let p = vector_range(-1.0, 1.0);
		let length_squared = p.magnitude_squared();
		if f32::EPSILON < length_squared && length_squared <= 1.0 {
			return p / length_squared.sqrt();
		}
	}
}
