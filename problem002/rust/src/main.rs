fn fibs(max: u32) -> Vec<u32> {
    // init fib vector
    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.push(2);

    let mut last = v[v.len() - 1];
    while last < max{
        v.push(v[v.len() - 1] + v[v.len() - 2]);
        last = v[v.len() - 1];
    }

    v
}

fn main() {
    let mut answer = 0;

    for fib in fibs(4_000_000) {
        if fib % 2 == 0 {
            answer += fib;
        }
    }

    println!("The answer is: {}", answer);
}
