fn parse_triangle(triangle: &str) -> Vec<Vec<u32>> {
    triangle
        .lines()
        .map(|l| l.trim().split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}

fn get_answer(triangle: &str) -> u32 {
    let mut triangle = parse_triangle(triangle);
    let mut previous_row: Option<Vec<u32>> = None;

    for row in triangle.iter_mut() {
        for (i, num) in row.iter_mut().enumerate() {
            let left = match previous_row {
                Some(ref r) => {
                    match i {
                        0 => 0,
                        _ => match r.get(i - 1) {
                            Some(n) => *n,
                            None => 0,
                        }
                    }
                },
                None => 0,
            };
            let right = match previous_row {
                Some(ref r) => match r.get(i) {
                    Some(n) => *n,
                    None => 0,
                },
                None => 0,
            };
            *num += [left, right].iter().max().unwrap();
        }
        previous_row = Some(row.to_vec());
    }
    *triangle[triangle.len() - 1].iter().max().unwrap()
}

fn main() {
    let triangle = "75
                    95 64
                    17 47 82
                    18 35 87 10
                    20 04 82 47 65
                    19 01 23 75 03 34
                    88 02 77 73 07 63 67
                    99 65 04 28 06 16 70 92
                    41 41 26 56 83 40 80 70 33
                    41 48 72 33 47 32 37 16 94 29
                    53 71 44 65 25 43 91 52 97 51 14
                    70 11 33 28 77 73 17 78 39 68 17 57
                    91 71 52 38 17 14 91 43 58 50 27 29 48
                    63 66 04 68 89 53 67 30 73 16 69 87 40 31
                    04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";
    println!("The answer is {}", get_answer(triangle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_triangle() {
        let triangle = "3
                        7 4
                        2 4 6
                        8 5 9 3";
        assert_eq!(parse_triangle(triangle), vec![
            vec![3],
            vec![7, 4],
            vec![2, 4, 6],
            vec![8, 5, 9, 3]]);
    }

    #[test]
    fn test_get_answer() {
        let triangle = "3
                        7 4
                        2 4 6
                        8 5 9 3";
        assert_eq!(get_answer(triangle), 23);
    }
}
