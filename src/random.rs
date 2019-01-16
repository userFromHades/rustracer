
use crate::vec_math::*;
use std::u32;

static mut seed : u32 = 1;

pub fn random () -> f32 {
	let a = 16807_u32;
	let m = 2147483647;
	let random = unsafe {
		seed = a.wrapping_mul(seed).wrapping_rem( m);
		seed
		};

	(random as f32) / 4294967295.0
}

pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let v = Vec3::new (random(), random(), random());
		if v.squre_length() < 1.0{
			return v;
		}
	}
}