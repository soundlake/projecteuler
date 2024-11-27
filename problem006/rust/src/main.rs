fn get_answer(n: u32) -> u32 {
    let mut sum_of_power: u32 = 0;
    let mut sum: u32 = 0;

    for i in 1..=n {
        sum += i;
        sum_of_power += i.pow(2);
    }

    sum.pow(2) - sum_of_power
}

fn main() {
    println!("The answer is {}", answer(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_with_10_is_2640() {
        let answer = get_answer(10);
        assert_eq!(answer, 2640);
    }
}
