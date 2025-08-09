use glm::{vec3, Vec3};
use rand::Rng;

const SIGN_EXP: u32 = 0x3F800000;

pub fn u32() -> u32 {
	rand::rng().random()
}
pub fn f32() -> f32 {
	f32::from_bits((u32() >> 9) | SIGN_EXP) - 1.0
}

pub fn f32_range(min: f32, max: f32) -> f32 {
	min + (max - min) * f32()
}
pub fn vector_range(min: f32, max: f32) -> Vec3 {
	vec3(f32_range(min, max), f32_range(min, max), f32_range(min, max))
}
pub fn vector() -> Vec3 {
	vec3(f32(), f32(), f32())
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
