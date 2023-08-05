use rand::{thread_rng, Rng};

pub fn is_prime(number: u128, rounds: usize) -> bool {
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

    for _ in 0..rounds {
        let a = rng.gen_range(2..=number - 2);
        let mut x = a.pow(d) % number;

        if x == 1 || x == number - 1 { return true; }

        for _ in 0..s {
            x = x.pow(2) % number;

            if x == number - 1 { return true; }
        }
    }

    false
}
