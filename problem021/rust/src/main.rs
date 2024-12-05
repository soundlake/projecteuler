fn find_proper_divisors(n: u16) -> Result<Vec<u16>, String> {
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

/// Returns the sum of proper divisors
fn d(n: u16) -> Result<u16, String> {
    match find_proper_divisors(n)?.into_iter().reduce(|a, b| a + b) {
        None => Ok(0),
        n => return Ok(n.unwrap()),
    }
}

fn is_amicable_number(n: u16) -> Result<bool, String> {
    let m = d(n)?;
    Ok(n != m && d(m)? == n)
}

fn get_answer() -> Result<u16, String> {
    Ok((2u16..10000)
        .filter(| &n | is_amicable_number(n).expect("0 or 1 is not acceptable"))
        .reduce(| a, b | a + b)
        .unwrap())
}

fn main() {
    println!("The answer is {}", get_answer().unwrap());
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
    fn test_d() {
        assert!(d(0).is_err());
        assert_eq!(d(1).unwrap(), 0);
        assert_eq!(d(2).unwrap(), 1);
        assert_eq!(d(10).unwrap(), 8);
        assert_eq!(d(220).unwrap(), 284);
    }

    #[test]
    fn test_is_amicable() {
        assert!(is_amicable_number(0).is_err());
        assert!(is_amicable_number(1).is_err());
        assert_eq!(is_amicable_number(2), Ok(false));
        assert_eq!(is_amicable_number(220), Ok(true));
    }
}
