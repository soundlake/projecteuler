use prime_db::get_prime_numbers_until;

fn get_answer(max: u32) -> u64 {
    let primes = get_prime_numbers_until(max, &mut [].to_vec());
    let mut sum: u64 = 0;
    for p in primes.into_iter() {
        sum += p as u64;
    }
    sum
}

fn main() {
    println!("The answer is {}", get_answer(2_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_answer(10), 17);
    }
}
