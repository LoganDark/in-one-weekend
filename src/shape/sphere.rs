use crate::math::space::Vec3;
use crate::math::ray::Ray;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::math::hit::HitResult;

#[derive(Debug)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
	pub material: Box<dyn Material>
}

impl Hittable for Sphere {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		// quadratic equation stuff, basically copied from the tutorial
		let diff: Vec3 = ray.pos - self.center;

		// if the ray is never going to hit, return early
		// this literally doubles the FPS with 500 spheres
		if diff.mag_sq() > self.radius * self.radius && ray.dir.dot(diff) >= 0. {
			return None
		}

		let a = ray.dir.mag_sq();
		let half_b = diff.dot(ray.dir);
		let c = diff.mag_sq() - self.radius * self.radius;
		let discriminant = half_b * half_b - a * c;

		if discriminant < 0. {
			return None
		}

		let root = discriminant.sqrt();

		let closest_t = (-half_b - root) / a;
		let furthest_t = (-half_b + root) / a;

		let t = if closest_t >= 0. { closest_t } else { furthest_t };

		if t < 0. || t * t > ray.length_squared() {
			None
		} else {
			Some(HitResult {
				ray: ray.clone(),
				t,
				normal: (ray.at(t) - self.center) / self.radius,
				material: self.material.as_ref()
			})
		}
	}

	fn is_inside(&self, point: Vec3) -> bool {
		(point - self.center).mag_sq() < self.radius * self.radius
	}
}