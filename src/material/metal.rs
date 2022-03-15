use crate::material::Material;
use crate::math::hit::HitResult;
use crate::math::color::Color;
use crate::hittable::Hittable;
use crate::material::lambertian::Lambertian;
use crate::material::albedo::solid::Solid;
use crate::material::albedo::Albedo;

#[derive(Copy, Clone, Debug)]
pub struct Metal<A: Albedo>(pub A, pub f64);

impl Metal<Solid> {
	pub fn new(roughness: f64) -> Self {
		Self::colored(Color(1., 1., 1.), roughness)
	}

	pub fn colored(color: Color, roughness: f64) -> Self {
		Metal(Solid(color), roughness)
	}
}

impl<A: Albedo> Albedo for Metal<A> {
	fn albedo(&self, result: HitResult) -> Color {
		self.0.albedo(result)
	}
}

impl<A: Albedo> Material for Metal<A> {
	fn color(&self, scene: &dyn Hittable, mut result: HitResult, reflections: u8) -> Color {
		let albedo = self.albedo(result);

		result.normal += Lambertian::<A>::random_vec() * self.1;
		result.normal.normalize();

		HitResult::get_color(
			scene.ray_trace(&result.reflected()),
			scene,
			reflections - 1
		) * albedo
	}
}
