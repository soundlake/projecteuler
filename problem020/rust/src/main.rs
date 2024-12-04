use digits::sum_of_digits;
use factorial::factorial;

fn get_answer(n: u32) -> u16 {
    sum_of_digits(factorial(n))
}

fn main() {
    println!("The answer is {}", get_answer(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(10), 27);
    }
}

