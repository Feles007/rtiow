use crate::hit_record::HitRecord;
use crate::ray::Ray;
use crate::rng;
use crate::utils::{near_zero, reflect, refract, Color};
use fml::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
	Lambertian { albedo: Color },
	Metal { albedo: Color, fuzz: f32 },
	Dielectric { refraction_index: f32 },
}
impl Material {
	pub fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<(Ray, Color)> {
		match self {
			Self::Lambertian { albedo } => {
				let mut scatter_direction = hit_record.normal + rng::unit_vector();

				if near_zero(scatter_direction) {
					scatter_direction = hit_record.normal;
				}
				Some((Ray::new(hit_record.point, scatter_direction), *albedo))
			},
			Self::Metal { albedo, fuzz } => {
				let mut reflected = reflect(ray.direction(), hit_record.normal);
				reflected = reflected.normalize() + (*fuzz * rng::unit_vector());
				let scattered = Ray::new(hit_record.point, reflected);
				if scattered.direction().dot(hit_record.normal) > 0.0 {
					Some((Ray::new(hit_record.point, reflected), *albedo))
				} else {
					//None
					Some((Ray::new(hit_record.point, reflected), *albedo))
				}
			},
			Self::Dielectric { refraction_index } => {
				let attenuation = Vec3::ONE;
				let ri = if hit_record.front_face {
					1.0 / refraction_index
				} else {
					*refraction_index
				};

				let unit_direction = ray.direction().normalize();
				let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
				let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

				let direction = if (ri * sin_theta > 1.0) || reflectance(cos_theta, ri) > rng::f32()
				// Cannot refract
				{
					reflect(unit_direction, hit_record.normal)
				} else {
					refract(unit_direction, hit_record.normal, ri)
				};

				let scattered = Ray::new(hit_record.point, direction);

				Some((scattered, attenuation))
			},
		}
	}
}
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
	let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
	r0 = r0 * r0;
	r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

type MaterialIdInner = u16;
#[derive(Debug, Copy, Clone)]
pub struct MaterialReference {
	id: MaterialIdInner,
}
impl MaterialReference {
	pub fn new(id: usize) -> Self {
		Self {
			id: MaterialIdInner::try_from(id).unwrap(),
		}
	}
	pub fn id(self) -> usize {
		self.id.into()
	}
}
