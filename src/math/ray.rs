use crate::math::space::Vec3;
use std::fmt::{Debug, Formatter};
use crate::material::lambertian::Lambertian;
use crate::material::albedo::solid::Solid;

#[derive(Copy, Clone, PartialEq)]
pub struct Ray {
	pub pos: Vec3,
	pub dir: Vec3
}

impl Ray {
	pub fn new(origin: Vec3, direction: Vec3) -> Self {
		Self { pos: origin, dir: direction }
	}
}

impl Debug for Ray {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Ray ({}, {}, {}) -> ({}, {}, {})",
			self.pos.x, self.pos.y, self.pos.z,
			self.dir.x, self.dir.y, self.dir.z
		)
	}
}

impl Default for Ray {
	fn default() -> Self {
		Ray::new(Vec3::new(0., 0., 0.), Vec3::default())
	}
}

impl Ray {
	#[inline]
	pub fn length_squared(&self) -> f64 {
		self.dir.mag_sq()
	}

	#[inline]
	pub fn length(&self) -> f64 {
		self.dir.mag()
	}

	#[inline]
	pub fn at(&self, t: f64) -> Vec3 {
		self.pos + self.dir * t
	}

	#[inline]
	pub fn from(&self, t: f64) -> Ray {
		Ray::new(self.at(t), self.dir * (1. - t))
	}

	pub fn t_for(&self, point: Vec3) -> f64 {
		//((point - self.pos).dot(self.dir).sqrt() / self.length()).powi(2)
		(point - self.pos).dot(self.dir) / self.length_squared()
	}

	#[inline]
	pub fn reflect(&self, t: f64, normal: Vec3) -> Ray {
		let mut reflected = self.from(t);

		reflected.dir -= normal * (reflected.dir.dot(normal) * 2.).min(0.);
		reflected.from(1e-10)
	}

	pub fn wobbled(&self, amount: f64) -> Ray {
		Ray {
			pos: self.pos,
			dir: (self.dir + (Lambertian::<Solid>::random_vec() * amount)).normalized()
		}
	}
}
