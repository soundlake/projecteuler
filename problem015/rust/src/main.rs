fn count_routes(width: usize, height: usize) -> u64 {
    // initialize the grid
    let mut grid = vec![vec![0u64; width + 1]; height + 1];
    for i in 0..(width + 1) { grid[0][i] = 1; }
    for i in 0..(height + 1) { grid[i][0] = 1; }

    for w in 1..(width + 1) {
        for h in 1..(height + 1) {
            grid[h][w] = grid[h - 1][w] + grid[h][w - 1];
        }
    }

    grid[height][width]
}

fn main() {
    println!("The answer is {}", count_routes(20, 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_routes() {
        assert_eq!(count_routes(1, 1), 2);
        assert_eq!(count_routes(1, 2), 3);
        assert_eq!(count_routes(2, 1), 3);
        assert_eq!(count_routes(2, 2), 6);
    }
}
