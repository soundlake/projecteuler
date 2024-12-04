use num_bigint::BigUint;

pub fn factorial(n: u32) -> BigUint{
    let mut f = BigUint::parse_bytes(b"1", 10).unwrap();
    for i in 1..=n {
        f *= i;
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(2), BigUint::parse_bytes(b"2", 10).unwrap());
        assert_eq!(factorial(3), BigUint::parse_bytes(b"6", 10).unwrap());
        assert_eq!(factorial(4), BigUint::parse_bytes(b"24", 10).unwrap());
        assert_eq!(factorial(5), BigUint::parse_bytes(b"120", 10).unwrap());
        assert_eq!(factorial(6), BigUint::parse_bytes(b"720", 10).unwrap());
    }
}
