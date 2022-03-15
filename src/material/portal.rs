use crate::math::space::Pos;
use crate::hittable::Hittable;
use crate::math::ray::Ray;
use crate::material::Material;
use crate::math::color::Color;
use crate::math::hit::HitResult;
use crate::material::albedo::Albedo;

#[derive(Copy, Clone, Debug)]
pub struct Portal(pub Pos);

impl Albedo for Portal {
	fn albedo(&self, _result: HitResult) -> Color {
		Color::splat(1.)
	}
}

impl Material for Portal {
	fn color(&self, scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color {
		let cut = result.ray.from(result.t + 1e-10);

		if result.is_back() {
			HitResult::get_color(scene.ray_trace(&cut), scene, reflections)
		} else {
			let new = Ray::new(self.0 * cut.pos, cut.dir.rotated_by(self.0.rotation));
			HitResult::get_color(scene.ray_trace(&new), scene, reflections - 1)
		}
	}
}
