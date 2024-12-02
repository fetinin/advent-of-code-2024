use std::io::{self, BufRead};

/// Reads lines from stdin and returns pairs
/// Example input:
/// 3   4
/// 4   3
/// 2   5
/// 1   3
/// 3   9
/// 3   3
pub fn read_pairs_from_stdin() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse integer"))
            .collect();

        if !parts.len() == 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid input format"));
        }

        first_list.push(parts[0]);
        second_list.push(parts[1]);
    }

    Ok((first_list, second_list))
}
