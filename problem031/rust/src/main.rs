fn get_answer(sum: &u32, coins: &[u32]) -> u32 {
    let len = coins.len();

    if len <= 0 { return 0; }
    if len == 1 {
        if sum % coins[0] == 0 { return 1; }
        return 0;
    }

    let mut current: u32 = 0;
    let mut i: u32 = 0;
    while *sum >= coins[len - 1] * i {
        current += get_answer(&(sum - (coins[len - 1] * i)), &coins[0..len - 1]);
        i += 1;
    }

    return current;
}

fn main() {
    let coins: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    println!("The answer is: {}", get_answer(&200, &coins));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_one_if_only_one() {
        assert_eq!(get_answer(&1, &[1]), 1);
        assert_eq!(get_answer(&2, &[1]), 1);
        assert_eq!(get_answer(&5, &[1]), 1);
        assert_eq!(get_answer(&10, &[1]), 1);
        assert_eq!(get_answer(&20, &[1]), 1);
        assert_eq!(get_answer(&50, &[1]), 1);
        assert_eq!(get_answer(&100, &[1]), 1);
        assert_eq!(get_answer(&200, &[1]), 1);
    }

    #[test]
    fn none_if_it_does_not_fit() {
        assert_eq!(get_answer(&5, &[2]), 0);
        assert_eq!(get_answer(&5, &[2, 4]), 0);
        assert_eq!(get_answer(&31, &[2, 4, 8, 16]), 0);
    }

    #[test]
    fn some_combination() {
        assert_eq!(get_answer(&4, &[1, 2]), 3);
        assert_eq!(get_answer(&5, &[1, 2]), 3);
        assert_eq!(get_answer(&6, &[1, 2]), 4);
        assert_eq!(get_answer(&5, &[1, 2, 5]), 4);
    }
}
