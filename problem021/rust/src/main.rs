use divisors::sum_of_proper_divisors;

fn is_amicable_number(n: u16) -> Result<bool, String> {
    let m = sum_of_proper_divisors(n)?;
    Ok(n != m && sum_of_proper_divisors(m)? == n)
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
    fn test_is_amicable() {
        assert!(is_amicable_number(0).is_err());
        assert!(is_amicable_number(1).is_err());
        assert_eq!(is_amicable_number(2), Ok(false));
        assert_eq!(is_amicable_number(220), Ok(true));
    }
}
