fn is_pythagorean_set(mut set: [u32; 3]) -> bool {
    set.sort();
    set[0].pow(2) + set[1].pow(2) == set[2].pow(2)
}

fn find_pythagorean_set(sum: u32) -> Result<[u32; 3], [u32; 0]> {
    for a in 1..(sum / 3) {
        for b in (sum / 3)..(sum * 2 / 3) {
            let c = sum - a - b;
            if is_pythagorean_set([a, b, c]) {
                return Ok([a, b, c]);
            }
        }
    }
    Err([])
}

fn main() {
    let the_set = find_pythagorean_set(1000);
    match the_set {
        Ok(s) => {
            let answer = s[0] * s[1] * s[2];
            println!("The answer is: {}", answer)
        },
        Err(_) => println!("The question is wrong. There is no possible answer!")
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pythagorean_set() {
        assert_eq!(is_pythagorean_set([3, 4, 5]), true);
        assert_eq!(is_pythagorean_set([3, 4, 6]), false);
    }

    #[test]
    fn pythagorean_set_with_sum_of_12_is_3_4_5() {
        assert_eq!(find_pythagorean_set(12), Ok([3, 4, 5]));
        assert_eq!(find_pythagorean_set(13), Err([]));
    }
}
