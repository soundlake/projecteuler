fn is_palindrome(number: u32) -> bool {
    let string = number.to_string();
    let len = string.len();
    let chars: Vec<char> = string.chars().collect();
    for i in 0..(len / 2) {
        if chars[i] != chars[len - i - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let mut largest = 0;

    for a in (100..1000).rev() {
        for b in (100..1000).rev() {
            let current = a * b;
            if is_palindrome(current) && current > largest {
                largest = current;
            }
        }
    }

    println!("The answer is {}", largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(101), true);
        assert_eq!(is_palindrome(1010), false);
    }
}
