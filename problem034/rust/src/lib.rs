fn f(n: u32) -> u32 {
    (1..=n)
        .reduce(|a, b| a * b)
        .unwrap_or(1)
}

fn calc(n: u32) -> u32 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| f(d))
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

pub fn check(n: u32) -> bool {
    n == calc(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(f(0), 1);
        assert_eq!(f(1), 1);
        assert_eq!(f(2), 2);
        assert_eq!(f(3), 6);
        assert_eq!(f(4), 24);
        assert_eq!(f(5), 120);
        assert_eq!(f(6), 720);
        assert_eq!(f(7), 5040);
        assert_eq!(f(8), 40320);
        assert_eq!(f(9), 362880);
    }

    #[test]
    fn test_check() {
        assert_eq!(check(145), true);
        assert_eq!(check(141), false);
    }
}
