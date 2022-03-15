use crate::material::Material;
use crate::math::hit::HitResult;
use crate::math::color::Color;
use crate::hittable::Hittable;
use crate::material::albedo::Albedo;
use crate::material::lambertian::Lambertian;

#[derive(Copy, Clone, Debug)]
pub struct Glossy<A: Albedo>(pub A);

impl<A: Albedo> Albedo for Glossy<A> {
	fn albedo(&self, result: HitResult) -> Color {
		self.0.albedo(result)
	}
}

impl<A: Albedo> Material for Glossy<A> {
	fn color(&self, scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color {
		if Lambertian::<A>::random_vec().y > -0.5 {
			Lambertian::<A>::color(scene, result, reflections) * self.albedo(result)
		} else {
			HitResult::get_color(
				scene.ray_trace(&result.reflected()),
				scene,
				reflections - 1
			)
		}

		//let mut color = Lambertian::<A>::color(scene, result, reflections) * self.albedo(result);

		//let reflected_color = HitResult::get_color(
		//	scene.ray_trace(&result.reflected()),
		//	scene,
		//	reflections - 1
		//);

		//color += reflected_color * reflected_color.luminance().min(1.) * (1. + result.ray.dir.normalized().dot(result.normal_abs()));
		//color
	}
}
