use crate::math::color::Color;
use std::prelude::v1::Vec;

pub mod math;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod scene;
pub mod shape;
pub mod progressive;
pub mod rng;

pub fn test_pattern(width: usize, height: usize) -> Vec<Color> {
	let mut arr = Vec::with_capacity(width * height);

	for y in (0..height).rev() {
		for x in 0..width {
			let r = (x as f64) / (width - 1) as f64;
			let g = (y as f64) / (height - 1) as f64;
			let b = 0.25;

			arr.push(Color(r, g, b));
		}
	}

	arr
}
