pub fn find_proper_divisors(n: u16) -> Result<Vec<u16>, String> {
    let mut divisors: Vec<u16> = [].to_vec();

    match n {
        0 => return Err("Any number can be divisor of zero.".to_string()),
        1 => return Ok(divisors),
        _ => divisors.push(1),
    }

    for i in 2..=((n as f32).sqrt().floor() as u16) {
        if n % i == 0 {
            let d = n / i;
            divisors.push(i);
            if i != d {
                divisors.push(d);
            }
        }
    }
    divisors.sort();
    Ok(divisors)
}

pub fn sum_of_proper_divisors(n: u16) -> Result<u16, String> {
    match find_proper_divisors(n)?.into_iter().reduce(|a, b| a + b) {
        None => Ok(0),
        n => return Ok(n.unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_proper_divisors() {
        assert!(find_proper_divisors(0).is_err());
        assert_eq!(find_proper_divisors(1).unwrap(), [].to_vec());
        assert_eq!(find_proper_divisors(2).unwrap(), [1].to_vec());
        assert_eq!(find_proper_divisors(10).unwrap(), [1, 2, 5].to_vec());
        assert_eq!(find_proper_divisors(220).unwrap(), [1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110].to_vec());
    }

    #[test]
    fn test_sum_of_proper_divisors() {
        assert!(sum_of_proper_divisors(0).is_err());
        assert_eq!(sum_of_proper_divisors(1).unwrap(), 0);
        assert_eq!(sum_of_proper_divisors(2).unwrap(), 1);
        assert_eq!(sum_of_proper_divisors(10).unwrap(), 8);
        assert_eq!(sum_of_proper_divisors(220).unwrap(), 284);
    }
}
