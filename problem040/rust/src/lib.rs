fn c(n: usize) -> char {
    let mut s: String = "".to_string();
    let mut i: u32 = 0;

    while s.len() <= n {
        i += 1;
        s.push_str(&i.to_string());
    }

    s.chars().nth(n - 1).unwrap()
}

fn d(n: usize) -> u32 {
    c(n).to_digit(10).unwrap()
}

pub fn get_answer() -> u32 {
    d(1)
        * d(10)
        * d(100)
        * d(1000)
        * d(10000)
        * d(100000)
        * d(1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        assert_eq!(c(1), '1');
        assert_eq!(c(2), '2');
        assert_eq!(c(9), '9');
        assert_eq!(c(12), '1');
    }

    #[test]
    fn test_d() {
        assert_eq!(d(1), 1);
        assert_eq!(d(2), 2);
        assert_eq!(d(9), 9);
        assert_eq!(d(12), 1);
    }
}
