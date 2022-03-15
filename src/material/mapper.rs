use crate::material::Material;
use crate::math::hit::HitResult;
use crate::math::color::Color;
use crate::hittable::Hittable;
use crate::math::ray::Ray;
use crate::material::albedo::Albedo;

#[derive(Copy, Clone, Debug)]
pub struct Mapper;

impl Albedo for Mapper {
	fn albedo(&self, _result: HitResult) -> Color {
		Color::splat(1.)
	}
}

impl Material for Mapper {
	fn color(&self, scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color {
		let normal = result.normal;
		let emit = Ray::new(result.pos() + normal * 1e-10, normal * result.reflected().length());
		HitResult::get_color(scene.ray_trace(&emit), scene, reflections - 1)
	}
}
