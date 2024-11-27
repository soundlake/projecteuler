fn get_answer(threshold: u32) -> u32 {
    let sum = (1..threshold)
        .filter(| x | x % 3 == 0 || x % 5 == 0)
        .reduce(| a, b | a + b);

    match sum {
        Some(n) => n,
        None => 0
    }
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
