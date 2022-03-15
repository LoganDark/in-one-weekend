use crate::hittable::Hittable;
use crate::math::ray::Ray;
use crate::math::hit::HitResult;
use crate::math::space::Vec3;
use std::fmt::Debug;

#[derive(Debug)]
pub struct TestScene {
	pub objects: Vec<Box<dyn Hittable>>
}

impl TestScene {
	pub fn new() -> Self {
		TestScene { objects: vec![] }
	}
}

impl Hittable for TestScene {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		let mut hit: Option<HitResult> = None;

		for object in &self.objects {
			if let Some(this_hit) = object.ray_trace(ray) {
				assert!(this_hit.t >= 0., "HIT BEFORE START: {:#?}, {:#?}", object, this_hit);
				assert!(this_hit.t < 1., "HIT AFTER END: {:#?}, {:#?}", object, this_hit);

				if let Some(current) = &hit {
					if this_hit.t < current.t {
						hit = Some(this_hit)
					}
				} else {
					hit = Some(this_hit)
				}
			}
		}

		return hit
	}

	fn is_inside(&self, point: Vec3) -> bool {
		for object in &self.objects {
			if object.is_inside(point) {
				return true
			}
		}

		false
	}
}

#[derive(Debug)]
pub struct Couple<'a, A: Hittable, B: Hittable>(&'a A, &'a B);

impl<'a, A: Hittable, B: Hittable> Couple<'a, A, B> {
	pub fn new(a: &'a A, b: &'a B) -> Self {
		Self(a, b)
	}
}

impl<'a, A: Hittable, B: Hittable> Couple<'a, A, B> {
	pub fn ray_trace_tagged(&self, ray: &Ray) -> Option<(HitResult<'a>, bool)> {
		let o1: Option<HitResult<'a>> = self.0.ray_trace(ray);
		let o2: Option<HitResult<'a>> = self.1.ray_trace(ray);

		match o1 {
			Some(h1) => match o2 {
				Some(h2) => Some((h1.closer(h2), h2.t >= h1.t)),
				None => o1.map(|h| (h, false))
			}
			None => o2.map(|h| (h, true))
		}
	}

	pub fn ray_trace(&self, ray: &Ray) -> Option<HitResult<'a>> {
		self.ray_trace_tagged(ray).map(|(h, _)| h)
	}

	pub fn is_inside(&self, point: Vec3) -> bool {
		self.0.is_inside(point) || self.1.is_inside(point)
	}
}
