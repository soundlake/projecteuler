use std::cmp::{max, min};
use crate::fraction::Fraction;

mod fraction;

fn find_right_fractions() -> Vec<Fraction> {
    let mut vec: Vec<Fraction> = vec![];

    for num in 1010..=9899 {
        let a = (num % 10000) / 1000;
        let b = (num % 1000) / 100;
        let c = (num % 100) / 10;
        let d = num % 10;
        if let Some(f) = Fraction::new(a, b, c, d) {
            if f.check() {
                vec.push(f);
            }
        }
    }

    vec
}

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = min(a, b);
    let mut b = max(a, b);
    while a > 1 {
        (b, a) = (a, b % a);
    }
    b
}

fn get_answer() -> u32 {
    let fractions = find_right_fractions();
    let product_of_numerator: u32 = fractions.iter()
        .map(| f | f.numerator)
        .reduce(| x, y | x * y)
        .unwrap_or(1)
        ;
    let product_of_denominator: u32 = fractions.iter()
        .map(| f | f.denominator)
        .reduce(| x, y | x * y)
        .unwrap_or(1)
        ;
    let gcd = gcd(product_of_numerator, product_of_denominator);
    product_of_denominator / gcd
}

fn main() {
    println!("The answer is {}", get_answer());
}
