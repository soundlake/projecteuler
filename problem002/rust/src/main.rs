fn fibs(max: u32) -> Vec<u32> {
    // init fib vector
    let mut v: Vec<u32> = [1, 2].to_vec();

    while v[v.len() - 1] < max {
        v.push(v[v.len() - 1] + v[v.len() - 2]);
    }

    v
}

fn main() {
    let answer = fibs(4_000_000)
        .into_iter()
        .filter(| f | f % 2 == 0)
        .reduce(| a, b | a + b)
        .unwrap();

    println!("The answer is: {}", answer);
}
