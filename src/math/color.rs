use std::ops::{Add, Sub, AddAssign, SubAssign, Div, DivAssign, Mul, MulAssign};
use crate::math::space::Vec3;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Color(pub f64, pub f64, pub f64);

impl Add for Color {
	type Output = Color;

	fn add(self, rhs: Color) -> Self::Output {
		Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
	}
}

impl AddAssign for Color {
	fn add_assign(&mut self, rhs: Color) {
		self.0 += rhs.0;
		self.1 += rhs.1;
		self.2 += rhs.2;
	}
}

impl Sub for Color {
	type Output = Color;

	fn sub(self, rhs: Color) -> Self::Output {
		Color(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
	}
}

impl SubAssign for Color {
	fn sub_assign(&mut self, rhs: Color) {
		self.0 -= rhs.0;
		self.1 -= rhs.1;
		self.2 -= rhs.2;
	}
}

impl Mul for Color {
	type Output = Color;

	fn mul(self, rhs: Color) -> Self::Output {
		Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
	}
}

impl MulAssign for Color {
	fn mul_assign(&mut self, rhs: Color) {
		self.0 *= rhs.0;
		self.1 *= rhs.1;
		self.2 *= rhs.2;
	}
}

impl Div for Color {
	type Output = Color;

	fn div(self, rhs: Color) -> Self::Output {
		Color(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
	}
}

impl DivAssign for Color {
	fn div_assign(&mut self, rhs: Color) {
		self.0 /= rhs.0;
		self.1 /= rhs.1;
		self.2 /= rhs.2;
	}
}

impl Mul<f64> for Color {
	type Output = Color;

	fn mul(self, rhs: f64) -> Self::Output {
		Color(self.0 * rhs, self.1 * rhs, self.2 * rhs)
	}
}

impl MulAssign<f64> for Color {
	fn mul_assign(&mut self, rhs: f64) {
		self.0 *= rhs;
		self.1 *= rhs;
		self.2 *= rhs;
	}
}

impl Div<f64> for Color {
	type Output = Color;

	fn div(self, rhs: f64) -> Self::Output {
		Color(self.0 / rhs, self.1 / rhs, self.2 / rhs)
	}
}

impl DivAssign<f64> for Color {
	fn div_assign(&mut self, rhs: f64) {
		self.0 /= rhs;
		self.1 /= rhs;
		self.2 /= rhs;
	}
}

impl Color {
	pub fn splat(n: f64) -> Color {
		Color(n, n, n)
	}

	#[inline]
	pub fn from_vec(vec: &Vec3) -> Color {
		let normalized = vec.normalized();
		Color((normalized.x + 1.) / 2., (normalized.y + 1.) / 2., (normalized.z + 1.) / 2.)
	}

	// https://twitter.com/jimhejl/status/633777619998130176
	pub fn tone_map_filmic_hejl2015(&self, white_point: f64) -> Color {
		let comp = |h: f64| {
			let a = 1.425 * h + 0.05;
			(h * a + 0.004) / (h * (a + 0.55) + 0.0491) - 0.0821
		};

		let white_point = comp(white_point);

		return Color(
			comp(self.0) / white_point,
			comp(self.1) / white_point,
			comp(self.2) / white_point
		)
	}

	#[inline]
	pub fn to_srgb(&self) -> u32 {
		fn l2s(component: f64) -> f64 {
			if component > 0.0031308 {
				1.055 * component.powf(1. / 2.4) - 0.055
			} else {
				12.92 * component
			}
		}

		let r = self.0.max(0.).min(1.);
		let g = self.1.max(0.).min(1.);
		let b = self.2.max(0.).min(1.);

		//assert!(r <= 1., "HDR value passed to to_srgb: r={}", r);
		//assert!(g <= 1., "HDR value passed to to_srgb: g={}", g);
		//assert!(b <= 1., "HDR value passed to to_srgb: b={}", b);

		let r = l2s(r);
		let g = l2s(g);
		let b = l2s(b);

		//assert!(r >= 0., "Negative value output by sRGB transform: r={}", r);
		//assert!(g >= 0., "Negative value output by sRGB transform: g={}", g);
		//assert!(b >= 0., "Negative value output by sRGB transform: b={}", b);
		//assert!(r <= 1., "HDR value output by sRGB transform: r={}", r);
		//assert!(g <= 1., "HDR value output by sRGB transform: g={}", g);
		//assert!(b <= 1., "HDR value output by sRGB transform: b={}", b);

		let ri = (255.999 * r) as u32;
		let gi = (255.999 * g) as u32;
		let bi = (255.999 * b) as u32;

		ri << 16 | gi << 8 | bi
	}

	#[inline]
	pub fn lerp(&self, other: &Color, t: f64) -> Color {
		*self * (1. - t) + *other * t
	}

	pub fn lerp_mut(&mut self, other: &Color, t: f64) {
		let oneminus = 1. - t;

		self.0 = self.0 * oneminus + other.0 * t;
		self.1 = self.1 * oneminus + other.1 * t;
		self.2 = self.2 * oneminus + other.2 * t;
	}

	#[inline]
	pub fn powi(&self, pow: i32) -> Color {
		Color(self.0.powi(pow), self.1.powi(pow), self.2.powi(pow))
	}

	#[inline]
	pub fn powf(&self, pow: f64) -> Color {
		Color(self.0.powf(pow), self.1.powf(pow), self.2.powf(pow))
	}

	pub fn luminance(&self) -> f64 {
		self.0 * 0.2126 + self.1 * 0.7152 + self.2 * 0.0722
	}
}
