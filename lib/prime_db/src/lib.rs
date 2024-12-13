fn get_known_primes() -> Vec<u32> {
    [
       2,   3,   5,   7,  11,  13,  17,  19,  23,  29,
    ].to_vec()
}

pub fn get_prime_numbers(n: usize, primes: &mut Vec<u32>) -> Vec<u32> {
    if primes.len() == 0 {
        get_known_primes()
            .iter()
            .for_each(| p | primes.push(*p));
    }
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

pub fn get_prime_numbers_until(n: u32, primes: &mut Vec<u32>) -> Vec<u32> {
    if primes.len() == 0 {
        get_known_primes()
            .iter()
            .for_each(| p | primes.push(*p));
    }
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

pub fn is_prime(n: u32, primes: &mut Vec<u32>) -> bool {
    if n % 2 == 0 && n != 2 {
        return false;
    }
    let primes = get_prime_numbers_until(n, primes);
    primes.contains(&n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prime_numbers() {
        let mut primes: Vec<u32> = Vec::new();
        assert_eq!(get_prime_numbers(0, &mut primes), [].to_vec());
        assert_eq!(get_prime_numbers(1, &mut primes), [2].to_vec());
        assert_eq!(get_prime_numbers(10, &mut primes), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29].to_vec());
        assert_eq!(get_prime_numbers(11, &mut primes), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31].to_vec());
    }

    #[test]
    fn test_get_prime_numbers_until() {
        let mut primes: Vec<u32> = Vec::new();
        assert_eq!(get_prime_numbers_until(1, &mut primes), [].to_vec());
        assert_eq!(get_prime_numbers_until(10, &mut primes), [2, 3, 5, 7].to_vec());
        assert_eq!(get_prime_numbers_until(40, &mut primes), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37].to_vec());
        assert_eq!(get_prime_numbers_until(41, &mut primes), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41].to_vec());
    }

    #[test]
    fn test_is_prime() {
        let mut primes: Vec<u32> = Vec::new();
        assert_eq!(is_prime(10, &mut primes), false);
        assert_eq!(is_prime(41, &mut primes), true);
    }
}
