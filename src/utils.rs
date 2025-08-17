use fml::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn linear_to_gamma(mut c: Color) -> Color {
	if c.x() > 0.0 {
		c = c.with_x(c.x().sqrt());
	}
	if c.y() > 0.0 {
		c = c.with_y(c.y().sqrt());
	}
	if c.z() > 0.0 {
		c = c.with_z(c.z().sqrt());
	}
	c
}
pub fn near_zero(v: Vec3) -> bool {
	v.x().abs() < f32::EPSILON && v.y().abs() < f32::EPSILON && v.z().abs() < f32::EPSILON
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
	v - 2.0 * v.dot(n) * n
}
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
	let cos_theta = (-uv).dot(n).min(1.0);
	let r_out_perp = etai_over_etat * (uv + cos_theta * n);
	let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
	r_out_perp + r_out_parallel
}
