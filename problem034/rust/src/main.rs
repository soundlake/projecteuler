use rust::check;

const RANDOM_LIMIT: u32 = 10u32.pow(6);

fn get_answer() -> u32 {
    (11..RANDOM_LIMIT)
        .filter(|n| check(*n))
        .reduce(|a, b| a + b)
        .unwrap_or(0)
}

fn main() {
    println!("The answer is: {}", get_answer());
}
