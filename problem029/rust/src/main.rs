use std::collections::HashSet;

fn get_answer(a_limit: u16, b_limit: u16) -> usize {
    let mut set: HashSet<(u16, u16)> = HashSet::new();
    for a in 2..=a_limit {
        let translated = get_base(a);
        for b in 2..=b_limit {
            set.insert((translated.0, translated.1 * b));
        }
    }
    set.len()
}

/// Ugly hard coded one. Should be upgraded.
fn get_base(n: u16) -> (u16, u16) {
    match n {
        // power of 2
        4 => (2, 2),
        9 => (3, 2),
        16 => (2, 4),
        25 => (5, 2),
        32 => (2, 5),
        36 => (6, 2),
        49 => (7, 2),
        64 => (2, 6),
        81 => (3, 4),
        100 => (10, 2),
        // power of 3
        8 => (2, 3),
        27 => (3, 3),
        // the rest
        n => (n, 1),
    }
}

fn main() {
    println!("The answer is {:?}", get_answer(100, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(5, 5), 15);
    }
}
