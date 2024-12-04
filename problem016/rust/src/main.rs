use num_bigint::BigUint;
use digits::sum_of_digits;

fn get_answer(pow: u32) -> u16 {
    let number = BigUint::parse_bytes(b"2", 10).unwrap().pow(pow);
    sum_of_digits(number)
}

fn main() {
    println!("The answer is {}", get_answer(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_digits() {
        assert_eq!(get_answer(15), 26);
    }
}
