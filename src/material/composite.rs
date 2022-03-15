use crate::material::Material;
use crate::math::hit::HitResult;
use crate::math::color::Color;
use crate::hittable::Hittable;

#[derive(Copy, Clone, Debug)]
pub enum Composite<A: Material, B: Material> {
	Add(A, B),
	Sub(A, B),
	Div(A, B),
	Mul(A, B),
	Lerp(A, B, f64),
}

impl<A: Material, B: Material> Material for Composite<A, B> {
	fn color(&self, scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color {
		match self {
			Self::Add(a, b) | Self::Sub(a, b) |
			Self::Div(a, b) | Self::Mul(a, b) |
			Self::Lerp(a, b, _) => {
				let color_a = a.color(scene, result.clone(), reflections);
				let color_b = b.color(scene, result.clone(), reflections);

				match self {
					Self::Add(..) => color_a + color_b,
					Self::Sub(..) => color_a - color_b,
					Self::Div(..) => color_a / color_b,
					Self::Mul(..) => color_a * color_b,
					Self::Lerp(_, _, t) => color_a.lerp(&color_b, *t)
				}
			}
		}
	}
}