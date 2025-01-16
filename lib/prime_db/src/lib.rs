use std::sync::{LazyLock, Mutex};

const KNOWN_PRIMES: [u32; 10] = [
   2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
];
static PRIMES: LazyLock<Mutex<Vec<u32>>> = LazyLock::new(|| Mutex::new(Vec::from(KNOWN_PRIMES)));

pub fn get_primes(n: usize) -> Vec<u32> {
    let mut primes = PRIMES.lock().unwrap();
    let mut candidate = primes[primes.len() - 1];
    'loop_prime: while primes.len() < n {
        candidate += 2;
        for p in primes.iter() {
            if candidate % p == 0 {
                // candidate is not a prime number
                break;
            }
            if *p > ((candidate as f32).sqrt().floor() as u32) {
                // candidate is a prime number
                primes.push(candidate);
                continue 'loop_prime;
            }
        }
    }
    primes[..n].to_vec()
}

pub fn get_primes_until(n: u32) -> Vec<u32> {
    let mut primes = PRIMES.lock().unwrap();
    let mut candidate = primes[primes.len() - 1];
    'loop_prime: while candidate + 2 <= n {
        candidate += 2;
        for p in primes.iter() {
            if candidate % p == 0 {
                // candidate is not a prime number
                break;
            }
            if *p > ((candidate as f32).sqrt().floor() as u32) {
                // candidate is a prime number
                primes.push(candidate);
                continue 'loop_prime;
            }
        }
    }
    for (i, p) in primes.iter().enumerate() {
        if *p > n {
            return primes[..i].to_vec();
        }
    }
    primes.to_vec()
}

pub fn is_prime(n: u32) -> bool {
    if n % 2 == 0 && n != 2 {
        return false;
    }
    let primes = get_primes_until(n);
    primes.contains(&n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_primes() {
        assert_eq!(get_primes(0), [].to_vec());
        assert_eq!(get_primes(1), [2].to_vec());
        assert_eq!(get_primes(10), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29].to_vec());
        assert_eq!(get_primes(11), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31].to_vec());
    }

    #[test]
    fn test_get_primes_until() {
        assert_eq!(get_primes_until(1), [].to_vec());
        assert_eq!(get_primes_until(10), [2, 3, 5, 7].to_vec());
        assert_eq!(get_primes_until(40), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].to_vec());
        assert_eq!(get_primes_until(41), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41].to_vec());
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(41), true);
    }
}
