use std::io::{self, BufRead};

/// Reads lines from stdin and returns pairs
/// Example input:
/// 71 73 74 76 78 80 77
/// 78 81 84 87 87
pub fn read_reports_from_stdin() -> io::Result<Vec<Vec<i32>>> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse integer"))
            .collect();

        reports.push(report);
    }

    Ok(reports)
}
