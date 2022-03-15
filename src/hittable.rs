use crate::math::ray::Ray;
use std::fmt::Debug;
use crate::math::hit::HitResult;
use crate::math::space::Vec3;

pub trait Hittable: Send + Sync + Debug {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult>;

	fn is_inside(&self, point: Vec3) -> bool;
}

impl<T: Hittable> Hittable for &T {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		T::ray_trace(self, ray)
	}

	fn is_inside(&self, point: Vec3) -> bool {
		T::is_inside(self, point)
	}
}