use prime_db::get_primes;

fn get_answer(n: usize) -> u32 {
    let primes = get_primes(n);
    primes[n - 1]
}

fn main() {
    println!("The answer is {}", get_answer(10_001));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_6th_prime_is_13() {
        let answer = get_answer(6);
        assert_eq!(answer, 13);
    }

    #[test]
    fn the_11th_prime_is_31() {
        let answer = get_answer(11);
        assert_eq!(answer, 31);
    }
}
