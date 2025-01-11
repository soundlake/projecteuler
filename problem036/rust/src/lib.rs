const ONE_MILLION: u32 = 1_000_000;

fn is_palindromic(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn check(n: &u32) -> bool {
    is_palindromic(&format!("{n}")[..]) &&
        is_palindromic(&format!("{n:b}")[..])
}

pub fn get_answer() -> u32 {
    (1..ONE_MILLION)
        .filter(|i| check(i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindromic() {
        assert_eq!(is_palindromic("1"), true);
        assert_eq!(is_palindromic("11"), true);
        assert_eq!(is_palindromic("121"), true);
        assert_eq!(is_palindromic("1221"), true);
        assert_eq!(is_palindromic("123"), false);
        assert_eq!(is_palindromic("1231"), false);
    }

    #[test]
    fn test_check() {
        assert_eq!(check(&585), true);
    }
}
