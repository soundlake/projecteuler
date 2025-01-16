use measure_time::measure_time;
use prime_db::is_prime;

fn f(n: u32, a: i32, b: i32) -> i64 {
    (n.pow(2) as i64) + (a as i64 * n as i64) + b as i64
}

fn n(a: i32, b: i32) -> usize {
    let mut i: u32 = 0;

    loop {
        let f = f(i, a, b);
        if f <= 0 || !(is_prime(f as u32)) {
            break;
        }
        i += 1;
    }

    i as usize
}

fn get_answer() -> i32 {
    let mut answer: (i32, i32, usize) = (-999, -999, 0);

    for a in -999..1000 {
        for b in -999..1000 {
            let n = n(a, b);
            if answer.2 < n {
                answer = (a, b, n);
            }
        }
    }

    answer.0 * answer.1
}

fn main() {
    println!("The answer is {}", measure_time!(get_answer()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert_eq!(f(40, 1, 41), 1681);
        assert_eq!(f(41, 1, 41), 1763);
    }

    #[test]
    fn test_n() {
        assert_eq!(n(1, 41), 40);
        assert_eq!(n(-79, 1601), 80);
    }
}
