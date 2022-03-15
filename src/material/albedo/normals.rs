use crate::math::hit::HitResult;
use crate::math::color::Color;
use crate::material::albedo::Albedo;

#[derive(Copy, Clone, Debug)]
pub struct Normals();

impl Albedo for Normals {
	fn albedo(&self, result: HitResult) -> Color {
		Color::from_vec(&result.normal).powi(2)
	}
}
