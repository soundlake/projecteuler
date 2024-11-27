fn get_answer(threshold: u32) -> u32 {
    let mut sum:u32 = 0;

    for number in 1..threshold {
        if number % 3 == 0 || number % 5 == 0 {
            sum += number;
        }
    }

    sum
}

fn main() {
    println!("The answer is: {}", get_answer(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(10), 23);
    }
}
