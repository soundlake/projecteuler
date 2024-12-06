use num_bigint::BigUint;
use measure_time::measure_time;

/// Get nth element of Fibonacci sequence
fn fib(n: usize) -> BigUint {
    let mut a: BigUint = BigUint::from(0u8);
    let mut b: BigUint = BigUint::from(1u8);

    for _ in 0..n {
        (a, b) = (b.clone(), a + b);
    }

    a
}

fn get_answer(n: usize) -> usize {
    let limit: BigUint = BigUint::from(10u8).pow(n as u32 - 1) - BigUint::from(1u8);
    let mut i: usize = 1;

    while fib(i) < limit {
        i += 1;
    }

    i
}

fn main() {
    let answer = measure_time!(get_answer(1000));
    println!("The answer is {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(1), BigUint::from(1u8));
        assert_eq!(fib(2), BigUint::from(1u8));
        assert_eq!(fib(3), BigUint::from(2u8));
        assert_eq!(fib(4), BigUint::from(3u8));
        assert_eq!(fib(5), BigUint::from(5u8));
        assert_eq!(fib(6), BigUint::from(8u8));
        assert_eq!(fib(7), BigUint::from(13u8));
        assert_eq!(fib(8), BigUint::from(21u8));
        assert_eq!(fib(9), BigUint::from(34u8));
        assert_eq!(fib(10), BigUint::from(55u8));
        assert_eq!(fib(11), BigUint::from(89u8));
        assert_eq!(fib(12), BigUint::from(144u8));
    }

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(3), 12);
    }
}
