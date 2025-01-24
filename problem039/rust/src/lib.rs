use std::collections::HashSet;

fn sort(arr: [u32; 3]) -> [u32; 3] {
    let [a, b, c] = arr;
    if a < b && b < c { return [a, b, c]; }
    if a < c && c < b { return [a, c, b]; }
    if b < a && a < c { return [b, a, c]; }
    if b < c && c < a { return [b, c, a]; }
    if c < a && a < b { return [c, a, b]; }
    [c, b, a]
}

fn is_right_angle_triangle(sides: [u32; 3]) -> bool {
    let [a, b, c] = sides;  // assuming it is sorted
    a.pow(2) + b.pow(2) == c.pow(2)
}

fn check(p: u32) -> usize {
    let mut result: HashSet<[u32; 3]> = HashSet::new();
    for c in (p / 3)..=(p / 2) {
        for a in 1..=((p - c) / 2) {
            let b = p - c - a;
            let sorted = sort([a, b, c]);
            if is_right_angle_triangle(sorted) {
                result.insert(sorted);
            }
        }
    }
    //dbg!(&result);
    result.len()
}

pub fn get_answer() -> u32 {
    let mut max: usize = 0;
    let mut answer: u32 = 0;
    for p in 10..=1000 {
        let result = check(p);
        if max < result {
            max = result;
            answer = p;
            dbg!(p, result);
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(sort([1, 2, 3]), [1, 2, 3]);
        assert_eq!(sort([1, 3, 2]), [1, 2, 3]);
        assert_eq!(sort([2, 1, 3]), [1, 2, 3]);
        assert_eq!(sort([2, 3, 1]), [1, 2, 3]);
        assert_eq!(sort([3, 1, 2]), [1, 2, 3]);
        assert_eq!(sort([3, 2, 1]), [1, 2, 3]);
    }

    #[test]
    fn test_is_right_angle_triangle() {
        assert_eq!(is_right_angle_triangle([3, 4, 5]), true);
        assert_eq!(is_right_angle_triangle([3, 4, 4]), false);
    }

    #[test]
    fn test_check() {
        assert_eq!(check(120), 3);
    }
}
