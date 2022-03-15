use crate::math::color::Color;
use crate::hittable::Hittable;
use crate::math::ray::Ray;
use crate::math::space::{Pos, Vec3};
use crate::math::hit::HitResult;
use crate::rng::{get_wyrand, gen_wyrand};

#[cfg(not(debug_assertions))]
use rayon::iter::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};

#[cfg(not(debug_assertions))]
use rayon::slice::ParallelSlice;

pub struct Camera {
	pub pos: Pos,
	pub blur_pos: Option<Pos>
}

impl Camera {
	pub fn calculate_far_plane(pos: Pos, width: usize, height: usize) -> (Vec3, Vec3, Vec3) {
		let cam_right: Vec3 = Vec3::new(1., 0., 0.).rotated_by(pos.rotation);
		let cam_down: Vec3 = Vec3::new(0., -1., 0.).rotated_by(pos.rotation);

		let plane_dist = 5000.;
		let plane_min = 2500.;
		let plane_width;
		let plane_height;

		if width > height {
			plane_width = plane_min * width as f64 / height as f64;
			plane_height = plane_min;
		} else {
			plane_width = plane_min;
			plane_height = plane_min * height as f64 / width as f64;
		}

		let fp_right: Vec3 = cam_right * plane_width;
		let fp_down: Vec3 = cam_down * plane_height;
		let fp_topleft: Vec3 = pos * Vec3::new(0., 0., -plane_dist) - ((fp_right + fp_down) / 2.);

		(fp_topleft, fp_right, fp_down)
	}

	pub fn rays(pos: Pos, width: usize, height: usize) -> Vec<Ray> {
		let (fp_topleft, fp_right, fp_down) = Self::calculate_far_plane(pos, width, height);

		let mut rays = Vec::with_capacity(width * height);

		let wf = width as f64;
		let hf = height as f64;

		let rng = get_wyrand();

		//let progress = rng.gen::<f64>() / 5.;
		//let blur = Vec3::new(progress, progress / 2., 0.);
		//let plane_origin = plane_origin + blur;
		let origin = pos.translation;// + blur;

		let mut y = 0.;

		while y < hf {
			let mut x = 0.;

			while x < wf {
				let xr = (x + gen_wyrand(rng)) / wf;
				let yr = (y + gen_wyrand(rng)) / hf;

				let direction: Vec3 = fp_topleft + fp_right * xr + fp_down * yr;
				rays.push(Ray::new(origin, direction));

				x += 1.;
			}

			y += 1.;
		}

		rays
	}

	pub fn render_combined(&self, scene: &dyn Hittable, width: usize, height: usize) -> Vec<(Color, Color, Vec3)> {
		let rays = Self::rays(self.pos, width, height);

		const REFLECTIONS: u8 = 5;

		#[cfg(not(debug_assertions))]
			let iter = rays.into_par_iter();

		#[cfg(debug_assertions)]
			let iter = rays.into_iter();

		return iter
			.map(|r| scene.ray_trace(&r))
			.map(|r|
				(
					HitResult::get_color(r, scene, REFLECTIONS),
					r.map(|r| r.material.albedo(r)).unwrap_or_default(),
					r.map(|r| r.normal).unwrap_or_default(),
				)
			)
			.collect();
	}

	pub fn separate_color(combined: &[(Color, Color, Vec3)]) -> Vec<Color> {
		#[cfg(not(debug_assertions))]
			let iter = combined.par_iter();

		#[cfg(debug_assertions)]
			let iter = combined.iter();

		iter.map(|t| t.0).collect()
	}

	pub fn separate_albedo(combined: &[(Color, Color, Vec3)]) -> Vec<Color> {
		#[cfg(not(debug_assertions))]
			let iter = combined.par_iter();

		#[cfg(debug_assertions)]
			let iter = combined.iter();

		iter.map(|t| t.1).collect()
	}

	pub fn separate_normals(combined: &[(Color, Color, Vec3)]) -> Vec<Vec3> {
		#[cfg(not(debug_assertions))]
			let iter = combined.par_iter();

		#[cfg(debug_assertions)]
			let iter = combined.iter();

		iter.map(|t| t.2).collect()
	}

	pub fn separate_color_f32(buf: &[Color]) -> Vec<f32> {
		let mut out = Vec::with_capacity(buf.len() * 3);

		for color in buf {
			out.push(color.0 as f32);
			out.push(color.1 as f32);
			out.push(color.2 as f32);
		}

		out
	}

	pub fn separate_vec3_f32(buf: &[Vec3]) -> Vec<f32> {
		let mut out = Vec::with_capacity(buf.len() * 3);

		for vec3 in buf {
			out.push(vec3.x as f32);
			out.push(vec3.y as f32);
			out.push(vec3.z as f32);
		}

		out
	}

	pub fn unite_f32_color(buf: &[f32]) -> Vec<Color> {
		#[cfg(not(debug_assertions))]
			let chunks_exact = buf.par_chunks_exact(3);

		#[cfg(debug_assertions)]
			let chunks_exact = buf.chunks_exact(3);

		chunks_exact
			.map(|vec3| Color(vec3[0] as f64, vec3[1] as f64, vec3[2] as f64))
			.collect()
	}

	pub fn render_color(&self, scene: &dyn Hittable, width: usize, height: usize) -> Vec<Color> {
		Camera::separate_color(&self.render_combined(scene, width, height))
	}

	pub fn render_albedo(&self, scene: &dyn Hittable, width: usize, height: usize) -> Vec<Color> {
		Camera::separate_albedo(&self.render_combined(scene, width, height))
	}

	pub fn render_normals(&self, scene: &dyn Hittable, width: usize, height: usize) -> Vec<Vec3> {
		Camera::separate_normals(&self.render_combined(scene, width, height))
	}
}
