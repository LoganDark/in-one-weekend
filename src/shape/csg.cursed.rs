use crate::hittable::Hittable;
use crate::math::ray::Ray;
use crate::math::hit::HitResult;

#[derive(Copy, Clone, Debug)]
pub enum Operation {
	Union,
	Subtract,
	Intersection
}

impl Operation {
	pub fn apply<'a, S1: Hittable, S2: Hittable>(&self, ray: &Ray, s1: &'a S1, s2: &'a S2) -> Option<HitResult<'a>> {
		match self {
			Operation::Union => {
				let o1 = s1.ray_trace(ray);
				let o2 = s2.ray_trace(ray);

				if let Some(h1) = &o1 {
					if let Some(h2) = &o2 {
						let ocloser;
						let ofarther;
						let hcloser;
						let hfarther;
						let scloser: &'a dyn Hittable;

						if h1.t < h2.t {
							ocloser = o1;
							ofarther = o2;
							hcloser = h1;
							hfarther = h2;
							scloser = s1;
						} else {
							ocloser = o2;
							ofarther = o1;
							hcloser = h2;
							hfarther = h1;
							scloser = s2;
						}

						if hcloser.is_back() == hfarther.is_back() {
							if hcloser.is_back() {
								ofarther
							} else {
								ocloser
							}
						} else {
							if hcloser.is_back() {
								ocloser
							} else {
								scloser.ray_trace(&ray.from(hcloser.t + 1e-10))
									.map(|r| r.map_onto_super(ray))
							}
						}
					} else {
						o1
					}
				} else {
					o2
				}
			}
			Operation::Subtract => {
				let o1 = s1.ray_trace(ray);

				if let Some(h1) = o1 {
					let back = h1.is_back();

					let from;
					let thru = if back { ray } else {
						from = ray.from(h1.t + 1e-10);
						&from
					};

					let o2 = s2.ray_trace(thru);

					if let Some(h2) = o2 {
						let n2 = HitResult { normal: -h2.normal, ..h2 }.map_onto_super(ray);

						if h2.is_back() {
							if back {
								o2.filter(|h2| h2.t < h1.t)
							} else {
								s1.ray_trace(&h2.ray.from(h2.t)).map(|h|
									if n2.t < h.map_onto_super(ray).t { n2 } else { h }
								)
							}
						} else {
							if n2.t < h1.t {
								Some(n2)
							} else {
								o1
							}
						}
					} else {
						let o2 = s2.ray_trace(ray);

						if let Some(h2) = o2 {
							let n2 = HitResult { normal: -h2.normal, ..h2 }.map_onto_super(ray);

							if !back && n2.is_back() && n2.t > h1.t {
								None
							} else {
								o1
							}
						} else {
							o1
						}
					}
				} else {
					None
				}
			}
			_ => todo!()
		}
	}
}

#[derive(Debug)]
pub struct CSG<S1: Hittable, S2: Hittable>(Operation, S1, S2);

impl<S1: Hittable, S2: Hittable> CSG<S1, S2> {
	pub fn new(operation: Operation, s1: S1, s2: S2) -> Self {
		CSG(operation, s1, s2)
	}
}

impl<S1: Hittable, S2: Hittable> Hittable for CSG<S1, S2> {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		self.0.apply(ray, &self.1, &self.2)
	}
}