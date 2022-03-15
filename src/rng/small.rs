use std::cell::UnsafeCell;
use rand::rngs::SmallRng;
use rand::{SeedableRng, Rng};
use rand::distributions::Uniform;

thread_local!(
	static SMALLRNG: UnsafeCell<SmallRng> = UnsafeCell::new(SmallRng::from_entropy());
);

pub fn get_small() -> &'static mut SmallRng {
	SMALLRNG.with(|c| unsafe { &mut *c.get() })
}

pub fn gen_small(rng: &mut SmallRng) -> f64 {
	rng.gen()
}

pub fn gen_small_once() -> f64 {
	gen_small(get_small())
}

pub struct RngRangeSmall {
	rng: &'static mut SmallRng,
	dist: Uniform<f64>
}

impl RngRangeSmall {
	pub fn range(min: f64, max: f64) -> RngRangeSmall {
		RngRangeSmall { rng: get_small(), dist: Uniform::new(min, max) }
	}

	pub fn once(min: f64, max: f64) -> f64 {
		Self::range(min, max).gen()
	}

	pub fn gen(&mut self) -> f64 {
		self.rng.sample(self.dist)
	}
}

pub struct RngRangeSmallContainer {
	range: RngRangeSmall
}

impl RngRangeSmallContainer {
	pub fn new(range: RngRangeSmall) -> Self {
		Self { range }
	}

	pub fn range(min: f64, max: f64) -> Self {
		Self::new(RngRangeSmall::range(min, max))
	}

	pub fn get(&self) -> &'static mut RngRangeSmall {
		unsafe { &mut *(&self.range as *const _ as *mut _) }
	}

	pub fn gen(&self) -> f64 {
		self.get().gen()
	}
}
