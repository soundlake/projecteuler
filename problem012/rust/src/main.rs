fn nth_triangle(n: u64) -> u64 {
    match n {
        0 => 0,
        n => (1..=n).reduce(|a, b| a + b).unwrap()
    }
}

fn find_divisors(n: u64) -> Vec<u64> {
    match n {
        0 => [0].to_vec(),
        1 => [1].to_vec(),
        n => {
            let mut divisors: Vec<u64> = [1, n].to_vec();
            for i in 2..=((n as f64).sqrt().floor() as u64) {
                if n % i == 0 {
                    divisors.push(i);
                    divisors.push(n / i);
                }
            }
            divisors.sort();
            divisors
        }
    }
}

fn get_answer(threshold: usize) -> u64 {
    let mut n: u64 = 1;
    loop {
        let triangle = nth_triangle(n);
        if find_divisors(triangle).len() > threshold {
            return triangle;
        }
        n += 1;
    }
}

fn main() {
    println!("The answer is {}", get_answer(500));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_triangle() {
        assert_eq!(nth_triangle(1), 1);
        assert_eq!(nth_triangle(2), 3);
        assert_eq!(nth_triangle(3), 6);
        assert_eq!(nth_triangle(4), 10);
        assert_eq!(nth_triangle(5), 15);
        assert_eq!(nth_triangle(6), 21);
        assert_eq!(nth_triangle(7), 28);
    }

    #[test]
    fn test_find_divisors() {
        assert_eq!(find_divisors(1), [1].to_vec());
        assert_eq!(find_divisors(3), [1, 3].to_vec());
        assert_eq!(find_divisors(6), [1, 2, 3, 6].to_vec());
        assert_eq!(find_divisors(10), [1, 2, 5, 10].to_vec());
        assert_eq!(find_divisors(15), [1, 3, 5, 15].to_vec());
        assert_eq!(find_divisors(21), [1, 3, 7, 21].to_vec());
        assert_eq!(find_divisors(28), [1, 2, 4, 7, 14, 28].to_vec());
    }

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(5), 28);
    }
}
