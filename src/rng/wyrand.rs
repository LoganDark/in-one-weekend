use std::cell::UnsafeCell;
use nanorand::rand::WyRand;
use nanorand::RNG;

thread_local!(
	static WYRAND: UnsafeCell<WyRand> = UnsafeCell::new(WyRand::new());
);

pub fn get_wyrand() -> &'static mut WyRand {
	WYRAND.with(|c| unsafe { &mut *c.get() })
}

pub fn gen_wyrand(rng: &mut WyRand) -> f64 {
	const MAX: f64 = u64::MAX as f64 + 1.;
	u64::from_ne_bytes(rng.rand()) as f64 / MAX
}

pub fn gen_wyrand_once() -> f64 {
	gen_wyrand(get_wyrand())
}

pub struct RngRangeWyrand {
	rng: &'static mut WyRand,
	mul: f64,
	add: f64
}

impl RngRangeWyrand {
	pub fn range(min: f64, max: f64) -> RngRangeWyrand {
		RngRangeWyrand { rng: get_wyrand(), mul: (max - min), add: min }
	}

	pub fn once(min: f64, max: f64) -> f64 {
		Self::range(min, max).gen()
	}

	pub fn gen(&mut self) -> f64 {
		gen_wyrand(self.rng).mul_add(self.mul, self.add)
	}
}

pub struct RngRangeWyrandContainer {
	range: RngRangeWyrand
}

impl RngRangeWyrandContainer {
	pub fn new(range: RngRangeWyrand) -> Self {
		Self { range }
	}

	pub fn range(min: f64, max: f64) -> Self {
		Self::new(RngRangeWyrand::range(min, max))
	}

	pub fn get(&self) -> &'static mut RngRangeWyrand {
		unsafe { &mut *(&self.range as *const _ as *mut _) }
	}

	pub fn gen(&self) -> f64 {
		self.get().gen()
	}
}
