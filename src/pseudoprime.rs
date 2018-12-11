use std::u64;
use std::i64;

extern crate modular_arithmetic;
use modular_arithmetic::{mod_add, mod_sub, mod_abs, mod_mul, mod_exp};


//tests whether n is composite using integer a < n
//true => composite | false => maybe prime

pub fn miller_rabin_witness(n: u64, odd: u64, power2: u64, a: u64) -> bool {
	let mut a_power = mod_exp(a, odd, n);
	if a_power == 1 { return false }

	if a_power == n-1 { return false }

	for _i in 0..power2 {
		a_power = mod_exp(a_power, 2 ,n);
		if a_power == n-1 {return false}
	}
	return true 
}



//computes the idx number in lucas U sequence
fn lucas_seq_u_mod(p: u64, q: u64, idx: u64, modulus: u64) -> u64 {
	if idx == 0 {
		0
	} else if idx == 1 {
		1 
	} else {
		let mut minus1 = 1;
		let mut minus2 = 0;
		let mut current = p;
		for _i in 2..idx {
			//overflow subtraction
			let current_hold = mod_sub(mod_mul(p, minus1, modulus) as i64, mod_mul(q, minus2, modulus) as i64, modulus);
			minus2 = minus1;
			minus1 = current;
			current = current_hold;
		}

		return current;
	}
}

fn lucas_seq_v_mod(p: u64, q: u64, idx: u64, modulus: u64) -> u64 {
	if idx == 0 {
		2
	} else if idx == 1 {
		p
	} else {
		let mut minus1 = 2;
		let mut minus2 = p;
		let mut current = mod_add(p,2,modulus);
		for _i in 2..idx {
			let current_hold = mod_sub( mod_mul(p, minus1, modulus) as i64, mod_mul(q, minus2, modulus) as i64, modulus);
			minus2 = minus1;
			minus1 = current;
			current = current_hold;
		}

		return current;
	}
}

pub fn lucas_test(n: u64, odd: u64, even_pow: u32, p: i64, q: i64) -> bool {
	let p0 = mod_abs(p, n);
	let q0 = mod_abs(q, n);
	let u_val = lucas_seq_u_mod(p0, q0, odd, n);
	if u_val != 0 {
		return false
	}

	for r in 0..even_pow {
		let v_val = lucas_seq_v_mod(p0, q0, odd * 2_u64.pow(r), n);
		if v_val != 0 {
			return false
		}
	}

	return true
}