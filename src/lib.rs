use std::u64;

extern crate rand;
use rand::distributions::{Distribution, Uniform};
 
extern crate modular_arithmetic;
use modular_arithmetic::{mut_even_power, split_odd_even, jacobi_symbol};

mod pseudoprime;
use pseudoprime::{miller_rabin_witness, lucas_test};



//TODO: ensure # rounds < n; infinite loop otherwise
//true => composite | false => maybe prime
pub fn miller_rabin(n: u64, rounds: u8) -> bool {
	//manually check divisibility properties for small primes
	//improves efficiency
	match trial_division_test(&n) {
		Some(b) => return b,
		None => ()
	}
	
	let mut tested_numbers = Vec::with_capacity(rounds as usize);
	//uniform sample space
	let mut rng = rand::thread_rng();
	let sample_space = Uniform::from(2..n-1);

	//split n-1 into odd component and even power in 2 steps
	let mut odd_component = n-1;
	let power2 = mut_even_power(&mut odd_component); //mutate n-1 into odd component, return even power

	//loop until #rounds different numbers have been tried
	while tested_numbers.len() < rounds as usize {

		let test_num = sample_space.sample(&mut rng);
		//excludes numbers already tried
		if !tested_numbers.contains(&test_num){
			//run a single miller rabin test 
			let test_result = miller_rabin_witness(n, odd_component, power2, test_num);
			if test_result {
				return true
			} else {
				tested_numbers.push(test_num);
			}
		}
	}
	return false
}





//returns true if n is definitely composite and false if n is likely to be a prime
pub fn baillie_psw(n: u64) -> bool {
	//perform trial division
	match trial_division_test(&n) {
		Some(b) => return b,
		None => ()
	}

	//check if n is a square manually
	let possible_square = (n as f64).sqrt() as u64;
	if possible_square * possible_square == n {
		return true
	}

	if (possible_square + 1) * (possible_square + 1) == n {
		return true
	}

	//base 2 strong pseudoprime test
	let (odd, even_pow) = split_odd_even(n-1);

	if miller_rabin_witness(n, odd, even_pow, 2) {
		return true
	}

	//find D with jacobi symbol (D/n) = -1
	//guaranteed to terminate
	let mut d:i64 = 5;
	let mut d_is_neg = false;

	loop {
		if jacobi_symbol(d, n) == -1 {
			break()
		}

		if d_is_neg == true {
			d *= -1;
			d += 2;
			d_is_neg = !d_is_neg;
		} else {
			d += 2;
			d += -1;
			d_is_neg = !d_is_neg;
		}
	}

	let p = 1;
	let q = (1-d)/4;



	//lucas probable prime test with D, P Q and n
	return lucas_test(n, odd, even_pow as u32, p, q)
}


//checks if n is divisible by one of the primes in the collection
fn trial_division_test(n: &u64) -> Option<bool> {
	let primes = vec![2,3,5,7,11];
	let primes_iter = primes.iter();
	for prime in primes_iter {
		if n == prime {
			return Some(false)
		} else if n % prime == 0 {
			return Some(true)
		}
	}
	return None
}