use crate::math::space::{Vec3, Pos};
use crate::hittable::Hittable;
use crate::math::ray::Ray;
use crate::material::Material;
use crate::math::hit::HitResult;

#[derive(Debug)]
pub struct Plane {
	pub center: Vec3,
	pub normal: Vec3,
	pub material: Box<dyn Material>
}

impl Plane {
	fn intersect(center: Vec3, normal: Vec3, ray: &Ray) -> Option<f64> {
		let denom = normal.dot(ray.dir.normalized());
		let dist = (center - ray.pos).dot(normal) / denom;

		if dist < 0. || dist * dist >= ray.length_squared() {
			None
		} else {
			Some(dist / ray.length())
		}
	}
}

impl Hittable for Plane {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		Plane::intersect(self.center, self.normal, ray).map(|t| HitResult {
			ray: ray.clone(),
			t,
			normal: self.normal.clone(),
			material: self.material.as_ref()
		})
	}

	fn is_inside(&self, point: Vec3) -> bool {
		(point - self.center).dot(self.normal) < 0.
	}
}

#[derive(Debug)]
pub struct FinitePlane {
	pub pos: Pos,
	pub size: (f64, f64),
	pub material: Box<dyn Material>
}

impl Hittable for FinitePlane {
	fn ray_trace(&self, ray: &Ray) -> Option<HitResult> {
		let normal = self.pos.rotation * Vec3::new(0., 1., 0.);

		Plane::intersect(
			self.pos.translation,
			normal,
			ray
		).and_then(|t| {
			let local: Vec3 = self.pos.inversed() * ray.at(t);

			if local.x.abs() * 2. > self.size.0 || local.z.abs() * 2. > self.size.1 {
				None
			} else {
				Some(HitResult {
					ray: ray.clone(),
					t,
					normal,
					material: self.material.as_ref()
				})
			}
		})
	}

	fn is_inside(&self, point: Vec3) -> bool {
		let rel = self.pos.inversed() * point;
		rel.y < 0. && rel.x.abs() * 2. < self.size.0 && rel.z.abs() * 2. < self.size.1
	}
}