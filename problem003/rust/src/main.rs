use measure_time::measure_time;

fn find_prime_factors(target: u64) -> Vec<u64> {
    let max = (target as f64).sqrt().floor() as u64;

    match target {
        0 | 1=> [].to_vec(),
        _ => {
            let mut pool: Vec<u64> = (2..(max + 1)).collect();
            let mut factors: Vec<u64> = Vec::new();

            while pool.len() > 0 {
                let candidate = pool[0];
                if target % candidate == 0 {
                    factors.push(candidate);
                }
                pool.retain(|i| i % candidate != 0);
            }

            // target is a prime number
            if factors.len() == 0 {
                return [target].to_vec();
            }

            factors
        }
    }
}

fn get_answer(target: u64) -> u64 {
    let factors = find_prime_factors(target);

    factors[factors.len() - 1]
}

fn main() {
    let answer = measure_time!(get_answer(600851475143));

    println!("The answer is: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_prime_factors() {
        assert_eq!(find_prime_factors(2), [2].to_vec());
        assert_eq!(find_prime_factors(3), [3].to_vec());
        assert_eq!(find_prime_factors(4), [2].to_vec());
        assert_eq!(find_prime_factors(13195), [5, 7, 13, 29].to_vec());
    }

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(13195), 29);
    }
}
