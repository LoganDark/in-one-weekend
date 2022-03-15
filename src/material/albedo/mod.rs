use std::fmt::Debug;
use crate::math::hit::HitResult;
use crate::math::color::Color;

pub mod normals;
pub mod solid;

pub trait Albedo: Send + Sync + Debug {
	fn albedo(&self, result: HitResult) -> Color;
}
