use std::collections::HashSet;
use rust::{BuildHasher, examine, filter};

fn main() {
    let digits = rust::get_digits();
    let mut products: HashSet<u32> = HashSet::new();
    for a in digits.clone().into_iter() {
        let digits = filter(&digits, &a);
        for b in digits.clone().into_iter() {
            let digits = filter(&digits, &b);
            for c in digits.clone().into_iter() {
                let digits = filter(&digits, &c);
                for d in digits.clone().into_iter() {
                    let digits = filter(&digits, &d);
                    for e in digits.clone().into_iter() {
                        let digits = filter(&digits, &e);
                        if let Some(product) = examine(&[a], &[b, c, d, e], &digits) {
                            products.insert(product);
                        }
                        if let Some(product) = examine(&[a, b], &[c, d, e], &digits) {
                            products.insert(product);
                        }
                    }
                }
            }
        }
    }

    let answer = products.clone().into_iter().reduce(|a, b| a + b).unwrap();
    println!("The answer is {answer}");
}
