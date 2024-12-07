use prime_db::get_prime_numbers_until;
use measure_time::measure_time;

/// Returns the longest cycle. Does not care about the
/// shortest cycle.
fn get_recurring_cycle(s: &str) -> Option<String> {
    let mut candidate: &str = &s[..(s.len() / 2)];
    while candidate.len() > 0 {
        if (1..(s.len() / candidate.len()))
            .all(| i | s[(i * candidate.len())..].starts_with(candidate)) {
            return Some(candidate.to_string());
        }
        candidate = &candidate[..(candidate.len() - 1)];
    }
    None
}

/// Only considers when d is prime.
fn recurring_cycle_of_unit_fraction(d: u32) -> String {
    let mut decimal_fraction: String = String::new();
    let mut numerator: u32 = 10;

    while numerator < d {
        decimal_fraction.push('0');
        numerator *= 10;
    }

    loop {
        numerator *= 10;
        decimal_fraction.push_str(&(numerator / d).to_string());
        numerator %= d;

        if numerator == 0 {
            return "".to_string();
        }

        match get_recurring_cycle(&decimal_fraction) {
            Some(s) => return s,
            None => (),
        }
    }
}

fn get_answer() -> u32 {
    let mut answer: (u32, usize) = (2, 0);
    for prime in get_prime_numbers_until(1000).into_iter() {
        let cycle = recurring_cycle_of_unit_fraction(prime);
        let len = cycle.len();
        if len > answer.1 {
            answer = (prime, len);
        }
    }
    answer.0
}

fn main() {
    println!("The answer is {:?}", measure_time!(get_answer()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_recurring_cycle() {
        assert_eq!(get_recurring_cycle("1"), None);
        assert_eq!(get_recurring_cycle("14"), None);
        assert_eq!(get_recurring_cycle("141"), None);
        // This function does not return the shortest cycle.
        assert_eq!(get_recurring_cycle("3333"), Some("33".to_string()));
        assert_eq!(get_recurring_cycle("142857142857"), Some("142857".to_string()));
        assert_eq!(get_recurring_cycle("0099"), None);
        assert_eq!(get_recurring_cycle("00990099"), Some("0099".to_string()));
    }

    #[test]
    fn test_recurring_cycle_of_unit_fraction() {
        assert_eq!(recurring_cycle_of_unit_fraction(2), "");
        assert_eq!(recurring_cycle_of_unit_fraction(3), "3");
        assert_eq!(recurring_cycle_of_unit_fraction(7), "142857");
        assert_eq!(recurring_cycle_of_unit_fraction(13), "076923");
        assert_eq!(recurring_cycle_of_unit_fraction(101), "0099");
    }
}
