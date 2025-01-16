use prime_db::get_primes_until;

fn main() {
    const MAX: u32 = 20;
    let primes = get_primes_until(MAX);
    let mut answer = 1;
    for base in primes.iter() {
        let mut power = 1;
        while base.pow(power + 1) < MAX { power += 1; }
        answer *= base.pow(power);
    }
    println!("The answer is: {:?}", answer);
}
