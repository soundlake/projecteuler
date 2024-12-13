fn get_n_layers(n: u32) -> u32 {
    (n + 1) / 2
}

fn get_diagnols(n: u32) -> Vec<u32> {
    let mut diagnols: Vec<u32> = Vec::new();
    for i in 0..get_n_layers(n) {
        if i == 0 {
            diagnols.push(1);
            continue;
        }
        (0..4).for_each(| _ | {
            diagnols.push(diagnols[diagnols.len() - 1] + (i * 2));
        });
    }
    diagnols
}

fn get_answer(n: u32) -> u32 {
    get_diagnols(n)
        .into_iter()
        .reduce(| a, b | a + b)
        .unwrap()
}

fn main() {
    println!("The answer is {:?}", get_answer(1001));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_n_layers() {
        assert_eq!(get_n_layers(1), 1);
        assert_eq!(get_n_layers(3), 2);
        assert_eq!(get_n_layers(5), 3);
    }

    #[test]
    fn test_get_diagnols() {
        assert_eq!(get_diagnols(1), [1].to_vec());
        assert_eq!(get_diagnols(3), [1, 3, 5, 7, 9].to_vec());
        assert_eq!(get_diagnols(5), [1, 3, 5, 7, 9, 13, 17, 21, 25].to_vec());
    }

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(1), 1);
        assert_eq!(get_answer(3), 25);
        assert_eq!(get_answer(5), 101);
    }
}
