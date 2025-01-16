use prime_db::{get_primes_until, is_prime};

fn is_truncatable_prime(n: u32) -> bool {
    n > 10
    &&
    get_l2r(n).iter().all(|p| is_prime(*p))
    &&
    get_r2l(n).iter().all(|p| is_prime(*p))
}

fn get_l2r(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![];
    let mut n_tenth: u32 = 10_u32.pow(n.to_string().len() as u32);

    loop {
        vec.push(n % n_tenth);
        n_tenth /= 10;
        if n_tenth < 10 { break; }
    }

    vec
}

fn get_r2l(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![];
    let mut n = n;

    loop {
        vec.push(n);
        n /= 10;
        if n < 1 { break; }
    }

    vec
}

fn found(n: usize) -> Vec<u32> {
    let mut limit = 100;
    let mut vec: Vec<u32> = vec![];
    let mut prime_i: usize = 0;

    while vec.len() < n {
        let primes = get_primes_until(limit);
        while prime_i < primes.len() {
            if is_truncatable_prime(primes[prime_i]) {
                vec.push(primes[prime_i]);
                if vec.len() >= n { return vec; }
            }
            prime_i += 1;
        }
        limit *= 10;
    }

    vec
}

pub fn get_answer() -> u32 {
    found(11).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l2r() {
        assert_eq!(get_l2r(37), vec![37, 7]);
        assert_eq!(get_l2r(3797), vec![3797, 797, 97, 7]);
    }

    #[test]
    fn test_r2l() {
        assert_eq!(get_r2l(37), vec![37, 3]);
        assert_eq!(get_r2l(3797), vec![3797, 379, 37, 3]);
    }

    #[test]
    fn test_is_truncatable_prime() {
        assert_eq!(is_truncatable_prime(2), false);
        assert_eq!(is_truncatable_prime(3), false);
        assert_eq!(is_truncatable_prime(5), false);
        assert_eq!(is_truncatable_prime(7), false);
        assert_eq!(is_truncatable_prime(37), true);
        assert_eq!(is_truncatable_prime(3797), true);
    }

    #[test]
    fn test_found() {
        assert_eq!(found(4), vec![23, 37, 53, 73]);
    }
}


