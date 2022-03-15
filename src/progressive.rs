use crate::math::color::Color;
use crate::math::space::Vec3;
use ultraviolet::Lerp;

pub struct ProgressiveBuffer {
	combined: Vec<(Color, Color, Vec3)>,
	spp: u64
}

impl ProgressiveBuffer {
	pub fn new(width: usize, height: usize) -> Self {
		Self {
			combined: vec![Default::default(); width * height],
			spp: 0
		}
	}

	pub fn update<T: AsRef<[(Color, Color, Vec3)]>>(&mut self, update: T) {
		let update = update.as_ref();
		assert_eq!(update.len(), self.combined.len(), "Buffer has an incorrect size!");

		self.spp += 1;
		let t = 1. / (self.spp as f64);

		self.combined.iter_mut()
			.zip(update)
			.for_each(|(t1, t2)| {
				t1.0.lerp_mut(&t2.0, t);
				t1.1.lerp_mut(&t2.1, t);
				t1.2 = t1.2.lerp(t2.2, t);
			});
	}

	pub fn spp(&self) -> u64 {
		self.spp
	}
}

impl AsRef<[(Color, Color, Vec3)]> for ProgressiveBuffer {
	fn as_ref(&self) -> &[(Color, Color, Vec3)] {
		&self.combined
	}
}
