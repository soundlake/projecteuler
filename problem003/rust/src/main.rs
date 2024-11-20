// https://stackoverflow.com/a/44378174/4276533 for measuring time
use std::time::{SystemTime, UNIX_EPOCH};

fn solution() {
    let target = 600851475143;
    let max = (target as f64).sqrt().floor() as u64;
    let mut pool: Vec<u64> = (2..(max + 1)).collect();
    let mut factors: Vec<u64> = Vec::new();

    while pool.len() > 0 {
        let candidate = pool[0];
        if target % candidate == 0 {
            factors.push(candidate);
        }
        pool.retain(|i| i % candidate != 0);
    }

    println!("The answer is: {}", factors[factors.len() - 1]);
}

fn main() {
    let start = SystemTime::now();

    solution();

    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("It took {:?}", since_the_epoch);
}
