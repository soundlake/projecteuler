// Returns set of integers that circular sibilings of the given n.
// n is not included.
pub fn circulars(n: &u32) -> Vec<u32> {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    let mut vec: Vec<u32> = vec![];
    for _ in 1..digits.len() {
        digits.rotate_left(1);
        let s: String = digits.iter().collect();
        let s: u32 = s.parse().unwrap();
        if s != *n { vec.push(s); }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circulars() {
        assert_eq!(circulars(&13), [31]);
        assert_eq!(circulars(&123), [231, 312]);
    }
}
