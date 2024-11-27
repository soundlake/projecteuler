fn get_known_primes() -> Vec<u32> {
    [
       2,   3,   5,   7,  11,  13,  17,  19,  23,  29,
    ].to_vec()
}

pub fn get_prime_numbers(n: usize) -> Vec<u32> {
    let mut primes = get_known_primes();
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

pub fn get_prime_numbers_until(n: u32) -> Vec<u32> {
    let mut primes = get_known_primes();
    let mut candidate = primes[primes.len() - 1];
    'loop_prime: while candidate + 2 < n {
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
    primes
}
