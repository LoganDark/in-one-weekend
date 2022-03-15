use crate::material::albedo::Albedo;
use crate::hittable::Hittable;
use crate::math::hit::HitResult;
use crate::math::color::Color;

pub mod albedo;

pub mod lambertian;
pub mod portal;
pub mod metal;
pub mod glossy;
pub mod mapper;
pub mod emissive;

pub trait Material: Albedo {
	fn color(&self, scene: &dyn Hittable, result: HitResult, reflections: u8) -> Color;
}
