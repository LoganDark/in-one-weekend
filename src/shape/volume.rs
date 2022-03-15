use crate::hittable::Hittable;
use crate::math::hit::HitResult;
use crate::math::space::Vec3;
use crate::math::ray::Ray;
use crate::rng::{RngRangeWyrand, gen_wyrand_once};
use crate::material::lambertian::Lambertian;
use crate::material::albedo::solid::Solid;

#[derive(Debug)]
pub struct Volume<H: Hittable>(pub H, pub f64);

impl<H: Hittable> Hittable for Volume<H> {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		let hit = self.0.ray_trace(ray)?;
		let start_t;
		let end_t;

		if self.0.is_inside(ray.pos) {
			start_t = 0.;
			end_t = hit.t;
		} else {
			start_t = hit.t;
			end_t = self.0.ray_trace(&ray.from(start_t + 1e-10))?.map_onto_super(ray).t;
		}

		assert!(end_t > start_t, "end_t ({}) < start_t ({})", end_t, start_t);

		let guaranteed_t = self.1 / ray.length();
		let dist_t = end_t - start_t;

		if gen_wyrand_once() < dist_t / guaranteed_t {
			let t = RngRangeWyrand::once(start_t, end_t);
			let mut normal = Lambertian::<Solid>::random_vec();

			if normal.dot(ray.dir) > 0. {
				normal = -normal;
			}

			Some(HitResult { ray: *ray, t, normal, material: hit.material })
		} else {
			None
		}
	}

	fn is_inside(&self, point: Vec3) -> bool {
		self.0.is_inside(point)
	}
}
