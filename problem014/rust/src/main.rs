fn count_seq(start: u64) -> u64 {
    let mut count: u64 = 0;
    let mut n: u64 = start;

    loop {
        count += 1;
        if n == 1 {
            return count;
        }
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    }
}

fn get_answer() -> u64 {
    let mut longest_count: u64 = 0;
    let mut answer: u64 = 0;

    for i in 13..1_000_000 {
        let count = count_seq(i);
        if count > longest_count {
            longest_count = count;
            answer = i;
        }
    }

    answer
}

fn main() {
    println!("The answer is {}", get_answer());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_seq() {
        assert_eq!(count_seq(13), 10);
    }
}
