use std::collections::{HashSet, VecDeque};
use measure_time::measure_time;
use prime_db::get_prime_numbers_until;
use rust::circulars;

const ONE_MILLION: u32 = 1_000_000;

fn get_answer() -> usize {
    let mut primes: VecDeque<u32> = VecDeque::new();
    primes = get_prime_numbers_until(ONE_MILLION, &mut primes.into()).into();
    let mut circular_primes: HashSet<u32> = HashSet::new();
    loop {
        // Reducing the primes VecDequq to make contains() more efficient
        match primes.pop_front() {
            None => break,
            Some(p) => {
                if circular_primes.contains(&p) { continue; }
                let circulars: Vec<u32> = circulars(&p);
                if circulars.iter().all(|c| primes.contains(c)) {
                    circular_primes.insert(p);
                    circulars.iter().for_each(|c| { circular_primes.insert(*c); });
                }
            },
        }
    }
    circular_primes.len()
}

fn main() {
    println!("The answer is: {}", measure_time!(get_answer()));
}
