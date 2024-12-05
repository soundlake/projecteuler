use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Read the file
    let mut file = File::open("../names.txt")?;
    let mut names = String::new();
    file.read_to_string(&mut names)?;

    // Parse the names
    let mut names: Vec<&str> = names
        .split(&['"', ','][..])
        .filter(| s | !s.is_empty())
        .collect();
    names.sort();

    let names: Vec<u32> = names
        .iter()
        .enumerate()
        .map(| (i, s) | -> u32 {
            s
                .chars()
                .map(| c | (c as u32) - ('A' as u32) + 1)
                .map(| n | n * ((i as u32) + 1))
                .reduce(| a, b | a + b)
                .expect("empty string slice encountered")
        })
        .collect();

    let answer: u32 = names
        .into_iter()
        .reduce(| a, b | a + b)
        .unwrap();

    println!("The answer is {}", answer);
    Ok(())
}
