// upper limit was calculated. 9u32.pow(5) is 59049.
// the functioned value with six 9s less than 999_999
const UPPER_LIMIT: u32 = 9u32.pow(5) * 6;
const LOWER_LIMIT: u32 = 11;

fn main() {
    let answer: u32 = (LOWER_LIMIT..=UPPER_LIMIT)
        .filter(|num| {
            let fun: u32 = num
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap().pow(5))
                .reduce(|a, b| a + b).unwrap();
            return fun == *num;
        })
        .reduce(|a, b| a + b).unwrap();

    println!("The answer is: {answer}");
}
