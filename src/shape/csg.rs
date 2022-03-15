use crate::hittable::Hittable;
use crate::math::ray::Ray;
use crate::math::hit::HitResult;
use crate::math::space::Vec3;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::scene::Couple;

pub trait Operation: Send + Sync + Debug {
	fn ray_trace<'a, S1: Hittable, S2: Hittable>(ray: &Ray, s1: &'a S1, s2: &'a S2) -> Option<HitResult<'a>>;
	fn is_inside<'a, S1: Hittable, S2: Hittable>(point: Vec3, s1: &'a S1, s2: &'a S2) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub enum Union {}

impl Operation for Union {
	fn ray_trace<'a, S1: Hittable, S2: Hittable>(ray: &Ray, s1: &'a S1, s2: &'a S2) -> Option<HitResult<'a>> {
		let couple = Couple::new(s1, s2);

		let mut hit = couple.ray_trace(ray)?;
		let mut portion = *ray;

		while couple.is_inside(portion.pos) {
			hit = couple.ray_trace(&portion)?;
			portion = hit.after_t();
		}

		Some(hit)
	}

	fn is_inside<'a, S1: Hittable, S2: Hittable>(point: Vec3, s1: &'a S1, s2: &'a S2) -> bool {
		s1.is_inside(point) || s2.is_inside(point)
	}
}

#[derive(Copy, Clone, Debug)]
pub enum Subtract {}

impl Operation for Subtract {
	fn ray_trace<'a, S1: Hittable, S2: Hittable>(ray: &Ray, s1: &'a S1, s2: &'a S2) -> Option<HitResult<'a>> {
		let mut o1 = s1.ray_trace(ray);
		let mut o2 = s2.ray_trace(ray);

		// ignore all intersections inside of the object
		while let Some(h1) = &o1 {
			if s2.is_inside(h1.pos()) {
				o2 = s2.ray_trace(&h1.after_t());
				o1 = s1.ray_trace(&h1.after_t());
			} else {
				break
			}
		}

		o1 = o1.map(|h| h.map_onto_super(ray));
		o2 = o2.map(|h| h.map_onto_super(ray));

		let h1;
		let h2;

		if let Some(ih1) = &o1 {
			if let Some(ih2) = &o2 {
				h1 = ih1;
				h2 = ih2;
			} else {
				// o1 is guaranteed to not be cut away since all cut
				// portions were skipped
				return o1
			}
		} else {
			return None
		}

		let h2flipped = HitResult { normal: -h2.normal, ..*h2 };

		if h2.t < h1.t {
			if h2.is_back() {
				if s1.is_inside(h2.pos()) {
					Some(h2flipped)
				} else {
					o1
				}
			} else {
				if h1.is_back() {
					Some(h2flipped)
				} else {
					if s2.is_inside(h1.pos()) {
						s2.ray_trace(&ray.from(h1.t + 1e-10))
							.map(|h| h.map_onto_super(ray).flip())
							.filter(|h| s1.is_inside(h.pos()))
					} else {
						o1
					}
				}
			}
		} else {
			if h2.is_back() {
				if s1.is_inside(h2.pos()) {
					Some(h2flipped)
				} else {
					None
				}
			} else {
				o1
			}
		}
	}

	fn is_inside<'a, S1: Hittable, S2: Hittable>(point: Vec3, s1: &'a S1, s2: &'a S2) -> bool {
		s1.is_inside(point) && !s2.is_inside(point)
	}
}

#[derive(Debug)]
pub struct CSG<O: Operation, S1: Hittable, S2: Hittable>(S1, S2, PhantomData<O>);

impl<S1: Hittable, S2: Hittable> CSG<Union, S1, S2> {
	pub fn union(shape1: S1, shape2: S2) -> Self {
		CSG(shape1, shape2, PhantomData)
	}
}

impl<S1: Hittable, S2: Hittable> CSG<Subtract, S1, S2> {
	pub fn subtract(shape: S1, cut: S2) -> Self {
		CSG(shape, cut, PhantomData)
	}
}

impl<O: Operation, S1: Hittable, S2: Hittable> Hittable for CSG<O, S1, S2> {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		O::ray_trace(ray, &self.0, &self.1)
	}

	fn is_inside(&self, point: Vec3) -> bool {
		O::is_inside(point, &self.0, &self.1)
	}
}
