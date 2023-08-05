use num_bigint::BigUint;
use rand::{thread_rng, Rng};

pub fn is_prime(number: u64, rounds: usize) -> bool {
    if number == 2 || number == 3 {
        return true;
    }

    if number % 2 == 0 || number < 2 {
        return false;
    }

    let mut rng = thread_rng();
    let mut s = 0;
    let mut d = (number - 1) as u32;

    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }

    'outer: for _ in 0..rounds {
        let a = BigUint::from(rng.gen_range(2..=number - 2));
        let mut x: BigUint = a.pow(d) % number;

        if x == BigUint::from(1u32) || x == BigUint::from(number - 1) {
            continue;
        }

        for _ in 0..s - 1 {
            x = x.pow(2) % number;

            if x == BigUint::from(number - 1) {
                continue 'outer;
            }
        }

        return false;
    }

    true
}
