#[derive(Debug, Copy, Clone)]
pub struct Interval {
	pub min: f32,
	pub max: f32,
}
impl Interval {
	pub const fn new(min: f32, max: f32) -> Self {
		Interval { min, max }
	}
	pub fn clamp(self, x: f32) -> f32 {
		if x < self.min {
			self.min
		} else if x > self.max {
			self.max
		} else {
			x
		}
	}
}
