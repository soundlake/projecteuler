use num::{ FromPrimitive, ToPrimitive, Unsigned };

const PERR: &str = "parse error";

pub fn find_proper_divisors<T: Unsigned + FromPrimitive + ToPrimitive + std::cmp::Ord + Copy>(n: T) -> Result<Vec<T>, String> {
    let mut divisors: Vec<T> = Vec::new();

    match n.to_u32().expect(PERR) {
        0 => return Err("Any number can be divisor of zero.".to_string()),
        1 => return Ok(divisors),
        _ => divisors.push(T::from_u32(1).expect(PERR)),
    }

    let mut i: T = T::from_u32(2).expect(PERR);
    while i <= (T::from_f32(T::to_f32(&n).expect(PERR).sqrt().floor()).expect(PERR)) {
        if n % i == T::from_u32(0).expect(PERR) {
            let d = n / i;
            divisors.push(i);
            if i != d {
                divisors.push(d);
            }
        }
        i = i + T::from_u32(1).expect(PERR);
    }
    divisors.sort();
    Ok(divisors)
}

pub fn sum_of_proper_divisors<T: Unsigned + FromPrimitive + ToPrimitive + std::cmp::Ord + Copy>(n: T) -> Result<T, String> {
    match find_proper_divisors(n)?.into_iter().reduce(|a, b| a + b) {
        None => Ok(T::from_u32(0).expect(PERR)),
        n => return Ok(n.unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_proper_divisors() {
        assert!(find_proper_divisors(0u8).is_err());
        assert_eq!(find_proper_divisors(1u8).unwrap(), [].to_vec());
        assert_eq!(find_proper_divisors(2u16).unwrap(), [1].to_vec());
        assert_eq!(find_proper_divisors(10u32).unwrap(), [1, 2, 5].to_vec());
        assert_eq!(find_proper_divisors(220u64).unwrap(), [1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110].to_vec());
    }

    #[test]
    fn test_sum_of_proper_divisors() {
        assert!(sum_of_proper_divisors(0u8).is_err());
        assert_eq!(sum_of_proper_divisors(1u8).unwrap(), 0);
        assert_eq!(sum_of_proper_divisors(2u16).unwrap(), 1);
        assert_eq!(sum_of_proper_divisors(10u32).unwrap(), 8);
        assert_eq!(sum_of_proper_divisors(220u64).unwrap(), 284);
    }
}
