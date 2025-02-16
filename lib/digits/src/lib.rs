use num_bigint::BigUint;

pub fn sum_of_digits(number: BigUint) -> u16 {
    number
        .to_str_radix(10)
        .chars()
        .map(| c | c.to_digit(10).unwrap() as u16)
        .reduce(| a, b | a + b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(sum_of_digits(BigUint::parse_bytes(b"2", 10).unwrap().pow(15)), 26);
    }
}
