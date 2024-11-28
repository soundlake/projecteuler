fn write_digit(n: &char) -> &str {
    match n {
        '1' => "one",
        '2' => "two",
        '3' => "three",
        '4' => "four",
        '5' => "five",
        '6' => "six",
        '7' => "seven",
        '8' => "eight",
        '9' => "nine",
        _ => "",
    }
}

fn write_tens(n: &char) -> &str {
    match n {
        '0' => "ten",
        '1' => "eleven",
        '2' => "twelve",
        '3' => "thirteen",
        '4' => "forteen",
        '5' => "fifteen",
        '6' => "sixteen",
        '7' => "seventeen",
        '8' => "eighteen",
        '9' => "nineteen",
        _ => "",
    }
}

fn write_second_digit(n: &char) -> &str {
    match n {
        '2' => "twenty",
        '3' => "thirty",
        '4' => "forty",
        '5' => "fifty",
        '6' => "sixty",
        '7' => "seventy",
        '8' => "eighty",
        '9' => "ninety",
        _ => "",
    }
}

fn write_out(n: u16) -> String {
    let digits: Vec<char> =
        // https://stackoverflow.com/a/50458236/4276533
        // How to pad with zero in the left
        format!("{:0>4}", n)
        .chars().rev().collect();
    let mut s = String::new();

    if digits[3] != '0' {
        // there is only one 4-digits number
        return "one thousand".to_string();
    }

    s.push_str(write_digit(&digits[2]));
    if s.len() > 0 {
        s.push(' ');
        s.push_str("hundred");
    }

    if digits[2] != '0' && (digits[1] != '0' || digits[0] != '0') {
        s.push_str(" and ");
    }

    if digits[1] == '1' {
        s.push_str(write_tens(&digits[0]));
        return s;
    }

    s.push_str(write_second_digit(&digits[1]));
    if digits[1] != '0' && digits[0] != '0' {
        s.push('-');
    }
    s.push_str(write_digit(&digits[0]));

    s
}

fn count(n: u16) -> usize {
    write_out(n)
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<Vec<char>>()
        .len()
}

fn get_answer(until: u16) -> u32 {
    let mut answer: u32 = 0;
    for i in 1..=until {
        answer += count(i) as u32;
    }
    answer
}

fn main() {
    println!("The answer is {}", get_answer(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_digit() {
        assert_eq!(write_digit(&'1'), "one".to_string());
        assert_eq!(write_digit(&'2'), "two".to_string());
        assert_eq!(write_digit(&'3'), "three".to_string());
        assert_eq!(write_digit(&'4'), "four".to_string());
        assert_eq!(write_digit(&'5'), "five".to_string());
        assert_eq!(write_digit(&'6'), "six".to_string());
        assert_eq!(write_digit(&'7'), "seven".to_string());
        assert_eq!(write_digit(&'8'), "eight".to_string());
        assert_eq!(write_digit(&'9'), "nine".to_string());
    }

    #[test]
    fn test_write_tens() {
        assert_eq!(write_tens(&'0'), "ten".to_string());
        assert_eq!(write_tens(&'1'), "eleven".to_string());
        assert_eq!(write_tens(&'2'), "twelve".to_string());
        assert_eq!(write_tens(&'3'), "thirteen".to_string());
        assert_eq!(write_tens(&'4'), "forteen".to_string());
        assert_eq!(write_tens(&'5'), "fifteen".to_string());
        assert_eq!(write_tens(&'6'), "sixteen".to_string());
        assert_eq!(write_tens(&'7'), "seventeen".to_string());
        assert_eq!(write_tens(&'8'), "eighteen".to_string());
        assert_eq!(write_tens(&'9'), "nineteen".to_string());
    }

    #[test]
    fn test_write_second_digit() {
        assert_eq!(write_second_digit(&'2'), "twenty".to_string());
        assert_eq!(write_second_digit(&'3'), "thirty".to_string());
        assert_eq!(write_second_digit(&'4'), "forty".to_string());
        assert_eq!(write_second_digit(&'5'), "fifty".to_string());
        assert_eq!(write_second_digit(&'6'), "sixty".to_string());
        assert_eq!(write_second_digit(&'7'), "seventy".to_string());
        assert_eq!(write_second_digit(&'8'), "eighty".to_string());
        assert_eq!(write_second_digit(&'9'), "ninety".to_string());
    }

    #[test]
    fn test_write_out() {
        assert_eq!(write_out(1), "one".to_string());
        assert_eq!(write_out(2), "two".to_string());
        assert_eq!(write_out(3), "three".to_string());
        assert_eq!(write_out(4), "four".to_string());
        assert_eq!(write_out(5), "five".to_string());
        assert_eq!(write_out(6), "six".to_string());
        assert_eq!(write_out(7), "seven".to_string());
        assert_eq!(write_out(8), "eight".to_string());
        assert_eq!(write_out(9), "nine".to_string());
        assert_eq!(write_out(342), "three hundred and forty-two".to_string());
        assert_eq!(write_out(115), "one hundred and fifteen".to_string());
    }

    #[test]
    fn test_count() {
        assert_eq!(count(1), 3);
        assert_eq!(count(342), 23);
        assert_eq!(count(115), 20);
    }

    #[test]
    fn test_get_answer() {
        assert_eq!(get_answer(5), 19);
    }
}
