use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub struct ObjectRange {
	start: usize,
	length: usize,
}
impl ObjectRange {
	pub fn new(start: usize, length: usize) -> Self {
		Self { start, length }
	}
	pub fn indices(&self) -> Range<usize> {
		self.start..self.start + self.length
	}
	pub fn length(&self) -> usize {
		self.length
	}
	pub fn split(&self) -> (Self, Self) {
		let l2 = self.length / 2;
		let left = Self::new(self.start, l2);
		let right = Self::new(self.start + l2, self.length - l2);
		assert_eq!(left.length + right.length, self.length);
		(left, right)
	}
}
