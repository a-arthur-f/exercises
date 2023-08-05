use num_bigint::BigUint;

pub fn factorial(number: u32) -> Result<BigUint, String> {
    if number < 2 {
        return Ok(BigUint::from(1u32));
    }

    let mut current_number = BigUint::from(1u8);
    let mut p = BigUint::from(1u8);
    let mut r = BigUint::from(1u8);
    let (mut h, mut shift, mut high) = (0i32, 0i32, 1i32);
    let mut log2_number = number.ilog2() as i32;
    let number = number as i32;

    while h != number {
        shift += h;
        h = number >> log2_number;
        log2_number -= 1;
        let mut len = high;
        high = (h - 1) | 1;
        len = (high - len) / 2;

        if len > 0 {
            p *= product(len, &mut current_number);
            r *= BigUint::new(p.to_u32_digits());
        }
    }

    Ok(r << shift)
}

fn product(number: i32, current_number: &mut BigUint) -> BigUint {
    let m = number / 2;

    if m == 0 {
        *current_number += BigUint::from(2u8);
        return BigUint::new(current_number.to_u32_digits());
    }

    if number == 2 {
        let result = (BigUint::new(current_number.to_u32_digits()) + BigUint::from(2u8))
            * (BigUint::new(current_number.to_u32_digits()) + BigUint::from(4u8));
        *current_number += BigUint::from(4u8);
        return result;
    }

    product(number - m, current_number) * product(m, current_number)
}
