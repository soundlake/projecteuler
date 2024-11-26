use prime_db::get_prime_numbers_until;

fn main() {
    const MAX: u32 = 20;
    let primes = get_prime_numbers_until(MAX);
    let mut answer = 1;
    for base in primes.iter() {
        let mut power = 1;
        while base.pow(power + 1) < MAX { power += 1; }
        answer *= base.pow(power);
    }
    println!("The answer is: {:?}", answer);
}
