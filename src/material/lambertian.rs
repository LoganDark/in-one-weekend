use crate::material::Material;
use crate::math::ray::Ray;
use crate::math::color::Color;
use crate::hittable::Hittable;
use crate::math::space::Vec3;
use std::f64::consts::PI;
use crate::math::hit::HitResult;
use crate::material::albedo::solid::Solid;
use crate::material::albedo::Albedo;
use crate::rng::RngRangeWyrandContainer;

#[derive(Copy, Clone, Debug)]
pub struct Lambertian<A: Albedo>(pub A);

impl Lambertian<Solid> {
	pub fn solid(color: Color) -> Self {
		Lambertian(Solid(color))
	}
}

thread_local!(
	static THREAD_2PI: RngRangeWyrandContainer = RngRangeWyrandContainer::range(0., PI * 2.);
	static THREAD_UNIT: RngRangeWyrandContainer = RngRangeWyrandContainer::range(-1., 1.);
);

impl<A: Albedo> Lambertian<A> {
	pub fn random_vec() -> Vec3 {
		let a = THREAD_2PI.with(RngRangeWyrandContainer::gen);
		let z = THREAD_UNIT.with(RngRangeWyrandContainer::gen);
		let r = (1f64 - z * z).sqrt();

		Vec3::new(r * a.cos(), r * a.sin(), z)
	}

	pub fn color(scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color {
		let remaining_length = result.ray.length() * (1. - result.t);
		let normal = result.normal_abs();

		let origin: Vec3 = result.pos() + normal * 1e-10;
		let direction = (normal + Lambertian::<A>::random_vec()).normalized() * remaining_length;

		HitResult::get_color(
			scene.ray_trace(&Ray::new(origin, direction)),
			scene,
			reflections - 1
		)
	}
}

impl<A: Albedo> Albedo for Lambertian<A> {
	fn albedo(&self, result: HitResult) -> Color {
		self.0.albedo(result)
	}
}

impl<A: Albedo> Material for Lambertian<A> {
	fn color(&self, scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color {
		Lambertian::<A>::color(scene, result, reflections) * self.albedo(result)
	}
}
