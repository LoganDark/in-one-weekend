use crate::math::hit::HitResult;
use crate::math::color::Color;
use crate::material::albedo::Albedo;

#[derive(Copy, Clone, Debug)]
pub struct Solid(pub Color);

impl Albedo for Solid {
	fn albedo(&self, _result: HitResult) -> Color {
		self.0.clone()
	}
}
