use crate::math::ray::Ray;
use crate::math::space::Vec3;
use crate::material::Material;
use crate::hittable::Hittable;
use crate::math::color::Color;

#[derive(Copy, Clone, Debug)]
pub struct HitResult<'a> {
	pub ray: Ray,
	pub t: f64,
	pub normal: Vec3,
	pub material: &'a dyn Material
}

impl<'a> HitResult<'a> {
	pub fn get_color(result: Option<HitResult>, scene: &dyn Hittable, reflections: u8) -> Color {
		if reflections == 0 {
			Color::default()
		} else {
			match result {
				Some(result) => result.material.color(scene, result, reflections),
				None => Color::default()
			}
		}
	}

	pub fn is_back(&self) -> bool {
		self.ray.dir.dot(self.normal) > 0.
	}

	pub fn normal_abs(&self) -> Vec3 {
		if self.is_back() {
			self.normal * -1.
		} else {
			self.normal.clone()
		}
	}

	pub fn reflected(&self) -> Ray {
		self.ray.reflect(self.t, self.normal_abs())
	}

	pub fn closer(self, other: HitResult<'a>) -> HitResult<'a> {
		if other.t < self.t {
			other
		} else {
			self
		}
	}

	pub fn farther(self, other: HitResult<'a>) -> HitResult<'a> {
		if other.t > self.t {
			other
		} else {
			self
		}
	}

	pub fn map_onto_super(self, ray: &Ray) -> HitResult<'a> {
		let t = ray.t_for(self.pos());
		HitResult { ray: ray.clone(), t, ..self }
	}

	pub fn pos(&self) -> Vec3 {
		self.ray.at(self.t)
	}

	pub fn flip(&self) -> HitResult<'a> {
		HitResult { normal: -self.normal, ..*self }
	}

	pub fn after_t(&self) -> Ray {
		self.ray.from(self.t + 1e-10)
	}

	pub fn wobbled(&self, amount: f64) -> HitResult<'a> {
		HitResult { ray: self.ray.wobbled(amount), ..*self }
	}
}
