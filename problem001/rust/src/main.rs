fn main() {
    let mut sum:u32 = 0;

    for number in 1..1000 {
        if number % 3 == 0 || number % 5 == 0 {
            sum += number;
        }
    }

    println!("The answer is: {}", sum);
}
