fn fact(n: usize) -> usize {
    match (1..=n).reduce(| a, b | a * b) {
        None => 1,
        Some(n) => n,
    }
}

fn nth_permutation(mut n: usize, mut pool: Vec<char>) -> String {
    let mut len: usize = pool.len();
    let mut answer = "".to_string();

    // sanity check
    if n == 0 {
        return pool.into_iter().collect();
    }

    // Translate 1.. to 0..
    n -= 1;

    while len > 0 {
        let fact = fact(len - 1);
        let i = n / fact;

        answer.push(pool.remove(i));

        len = pool.len();
        n %= fact;
    }

    answer
}

fn main() {
    let pool: Vec<char> = ('0'..='9').collect();
    let answer = nth_permutation(1_000_000, pool);
    println!("The answer is {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact() {
        assert_eq!(fact(0), 1);
        assert_eq!(fact(1), 1);
        assert_eq!(fact(2), 2);
        assert_eq!(fact(3), 6);
        assert_eq!(fact(4), 24);
        assert_eq!(fact(5), 120);
        assert_eq!(fact(6), 720);
        assert_eq!(fact(7), 5040);
        assert_eq!(fact(8), 40320);
        assert_eq!(fact(9), 362880);
    }

    #[test]
    fn test_nth_permutation() {
        assert_eq!(nth_permutation(1, vec!['0', '1', '2']), "012".to_string());
        assert_eq!(nth_permutation(2, vec!['0', '1', '2']), "021".to_string());
        assert_eq!(nth_permutation(3, vec!['0', '1', '2']), "102".to_string());
        assert_eq!(nth_permutation(4, vec!['0', '1', '2']), "120".to_string());
        assert_eq!(nth_permutation(5, vec!['0', '1', '2']), "201".to_string());
        assert_eq!(nth_permutation(6, vec!['0', '1', '2']), "210".to_string());
    }
}
