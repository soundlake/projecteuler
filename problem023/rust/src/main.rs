use divisors::sum_of_proper_divisors;

const LIMIT: u32 = 28123;

fn is_abundant_number(n: u32) -> bool {
    n < sum_of_proper_divisors(n).expect("could not get sum of proper divisors")
}

fn main() {
    let abundant_numbers: Vec<u32> = (1..=(LIMIT - 12))
        .filter(| n | is_abundant_number(*n))
        .collect();

    // positive integers which cannot be written as the sum of two abundant numbers.
    let mut numbers: Vec<u32> = (1..24).collect();

    'loop_candidate: for candidate in 24..LIMIT {
        for abundant in abundant_numbers.iter() {
            if candidate <= *abundant {
                break;
            }
            match abundant_numbers.binary_search(&(candidate - abundant)) {
                Ok(_) => continue 'loop_candidate,
                Err(_) => (),
            }
        }
        numbers.push(candidate);
    }

    println!("{:?}", numbers.into_iter().reduce(| a, b | a + b).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_abundant_number() {
        assert_eq!(is_abundant_number(4), false);
        assert_eq!(is_abundant_number(6), false);
        assert_eq!(is_abundant_number(12), true);
        assert_eq!(is_abundant_number(28), false);
    }
}
