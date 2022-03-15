use crate::material::albedo::Albedo;
use crate::math::hit::HitResult;
use crate::math::color::Color;
use crate::material::Material;
use crate::hittable::Hittable;
use crate::material::lambertian::Lambertian;
use crate::material::albedo::solid::Solid;

#[derive(Debug)]
pub struct Emissive<A: Albedo>(pub A, f64);

impl Emissive<Solid> {
	pub fn solid(color: Color) -> Self {
		let max_component = color.0.max(color.1).max(color.2).max(1.);
		Self(Solid(color / max_component), max_component)
	}
}

impl<A: Albedo> Albedo for Emissive<A> {
	fn albedo(&self, result: HitResult) -> Color {
		self.0.albedo(result)
	}
}

impl<A: Albedo> Material for Emissive<A> {
	fn color(&self, scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color {
		Lambertian::<A>::color(scene, result, reflections) + self.albedo(result) * self.1
	}
}
