pub fn get_answer() -> u32 {
    let mut max: u32 = 0;
    for x in 1..100000 {
        for n in 2..=9 {
            if let Some(result) = check(x, n) {
                if max < result {
                    //dbg!(x, n, result);
                    max = result;
                }
            }
        }
    }
    max
}

fn check(x: u32, n: u32) -> Option<u32> {
    // sanity check
    if n <= 1 || 9 < n { return None; }
    let result: u32 = (1..=n)
        .map(|n| x * n)
        .map(|n| n.to_string())
        .reduce(|a, b| a + &b)
        .unwrap()       // it is safe because iter never be empty
        .parse()
        .unwrap_or(0);  // handle error when the value is too big
    if is_1_to_9_pandigital(result) {
        Some(result)
    } else {
        None
    }
}

fn is_1_to_9_pandigital(n: u32) -> bool {
    if n < 100_000_000 { return false; }
    if 1000_000_000 <= n { return false; }
    let digits: Vec<char> = n.to_string().chars().collect();
    (1..=9).all(|digit| {
        digits.contains(&char::from_digit(digit, 10).unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_1_to_9_pandigital() {
        assert_eq!(is_1_to_9_pandigital(23), false);
        assert_eq!(is_1_to_9_pandigital(123456789), true);
        assert_eq!(is_1_to_9_pandigital(112345678), false);
        assert_eq!(is_1_to_9_pandigital(1234567890), false);
    }

    #[test]
    fn test_check() {
        assert_eq!(check(192, 3), Some(192384576));
        assert_eq!(check(9, 5), Some(918273645));
        assert_eq!(check(9, 1), None);
        assert_eq!(check(9, 10), None);
    }
}
