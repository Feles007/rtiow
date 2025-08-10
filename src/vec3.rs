use std::arch::x86_64::{
	__m128, _mm_add_ps, _mm_div_ps, _mm_extract_ps, _mm_max_ps, _mm_min_ps, _mm_movehl_ps, _mm_mul_ps, _mm_rcp_ps,
	_mm_shuffle_ps, _mm_sub_ps,
};
use std::mem::transmute;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
	inner: __m128,
}
impl Vec3 {
	pub const ZERO: Self = Self::splat(0.0);
	pub const ONE: Self = Self::splat(1.0);

	pub const fn new(x: f32, y: f32, z: f32) -> Self {
		let array = [x, y, z, 0.0];
		Self {
			inner: unsafe { transmute(array) },
		}
	}
	pub fn x(self) -> f32 {
		unsafe { f32::from_bits(_mm_extract_ps::<0>(self.inner) as u32) }
	}
	pub fn y(self) -> f32 {
		unsafe { f32::from_bits(_mm_extract_ps::<1>(self.inner) as u32) }
	}
	pub fn z(self) -> f32 {
		unsafe { f32::from_bits(_mm_extract_ps::<2>(self.inner) as u32) }
	}
	pub const fn splat(f: f32) -> Self {
		Self::new(f, f, f)
	}
	pub fn with_x(self, x: f32) -> Self {
		Self::new(x, self.y(), self.z())
	}
	pub fn with_y(self, y: f32) -> Self {
		Self::new(self.x(), y, self.z())
	}
	pub fn with_z(self, z: f32) -> Self {
		Self::new(self.x(), self.y(), z)
	}
	pub fn magnitude_squared(self) -> f32 {
		self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
	}
	pub fn magnitude(self) -> f32 {
		self.magnitude_squared().sqrt()
	}
	pub fn normalize(self) -> Self {
		self / self.magnitude()
	}
	pub fn dot(self, rhs: Self) -> f32 {
		self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
	}
	// https://geometrian.com/resources/cross_product/
	pub fn cross(self, rhs: Self) -> Self {
		let inner = unsafe {
			let tmp0 = _mm_shuffle_ps::<201>(self.inner, self.inner);
			let tmp1 = _mm_shuffle_ps::<210>(rhs.inner, rhs.inner);
			let tmp2 = _mm_mul_ps(tmp0, rhs.inner);
			let tmp3 = _mm_mul_ps(tmp0, tmp1);
			let tmp4 = _mm_shuffle_ps::<201>(tmp2, tmp2);
			_mm_sub_ps(tmp3, tmp4)
		};
		Self { inner }
	}
	#[inline]
	pub fn min(self, rhs: Self) -> Self {
		let inner = unsafe { _mm_min_ps(self.inner, rhs.inner) };
		Self { inner }
	}
	#[inline]
	pub fn max(self, rhs: Self) -> Self {
		let inner = unsafe { _mm_max_ps(self.inner, rhs.inner) };
		Self { inner }
	}
	pub fn reciprocal(self) -> Self {
		let inner = unsafe { _mm_rcp_ps(self.inner) };
		Self { inner }
	}
	pub fn horizontal_max(self) -> f32 {
		unsafe {
			let v1 = self.inner;
			let v2 = _mm_shuffle_ps::<85>(v1, v1);
			let max1 = _mm_max_ps(v1, v2);
			let v3 = _mm_movehl_ps(v1, v1);
			let max2 = _mm_max_ps(max1, v3);
			f32::from_bits(_mm_extract_ps::<0>(max2) as u32)
		}
	}
	pub fn horizontal_min(self) -> f32 {
		unsafe {
			let v1 = self.inner;
			let v2 = _mm_shuffle_ps::<85>(v1, v1);
			let max1 = _mm_min_ps(v1, v2);
			let v3 = _mm_movehl_ps(v1, v1);
			let max2 = _mm_min_ps(max1, v3);
			f32::from_bits(_mm_extract_ps::<0>(max2) as u32)
		}
	}
}
impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::new(-self.x(), -self.y(), -self.z())
	}
}
impl Add for Vec3 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let inner = unsafe { _mm_add_ps(self.inner, rhs.inner) };
		Self { inner }
	}
}
impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}
impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let inner = unsafe { _mm_sub_ps(self.inner, rhs.inner) };
		Self { inner }
	}
}
impl SubAssign for Vec3 {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs
	}
}
impl Mul for Vec3 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let inner = unsafe { _mm_mul_ps(self.inner, rhs.inner) };
		Self { inner }
	}
}
impl Mul<f32> for Vec3 {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
	}
}
impl Mul<Vec3> for f32 {
	type Output = Vec3;

	fn mul(self, rhs: Vec3) -> Self::Output {
		rhs * self
	}
}
impl Div for Vec3 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		let inner = unsafe { _mm_div_ps(self.inner, rhs.inner) };
		Self { inner }
	}
}
impl Div<f32> for Vec3 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		Self::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
	}
}
impl DivAssign<f32> for Vec3 {
	fn div_assign(&mut self, rhs: f32) {
		*self = *self / rhs
	}
}
