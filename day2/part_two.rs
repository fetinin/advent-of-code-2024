use std::cmp::Ordering;

mod tools;

// https://adventofcode.com/2024/day/2#part2
fn main() {
    let reports = tools::read_reports_from_stdin().expect("Failed to read lines");
    let safe_reports = count_safe_reports(reports);
    println!("{}", safe_reports.len());
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    reports.into_iter().filter(is_safe_report).collect()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    if check_report_is_good(report) {
        return true;
    }
    for i in 0..report.len() {
        let iter = report.iter().enumerate().
            filter(|&(j, _)| j != i).map(|(_, &v)| v);
        let filtered_report: Vec<i32> = iter.collect();
        if check_report_is_good(&filtered_report) {
            return true;
        }
    }

    false
}

fn check_report_is_good(report: &Vec<i32>) -> bool {
    let mut level_prev: i32 = report[0];
    let direction = report[0].cmp(&report[1]);
    if direction == Ordering::Equal {
        return false;
    }

    for level in report.iter().skip(1) {
        if level_prev.cmp(level).ne(&direction) {
            return false;
        }
        if *level == level_prev {
            return false;
        }
        if (level_prev - level).abs() > 3 {
            return false;
        }
        level_prev = *level;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // Safe because the levels are all decreasing by 1 or 2.
    #[test]
    fn test_one() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_safe_report(&report));
    }

    // Unsafe regardless of which level is removed.
    #[test]
    fn test_two() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_safe_report(&report));
    }

    // Unsafe regardless of which level is removed.
    #[test]
    fn test_three() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_safe_report(&report));
    }

    // Safe by removing the second level, 3.
    #[test]
    fn test_four() {
        let report: Vec<i32> = vec![1, 3, 2, 4, 5];
        assert!(is_safe_report(&report));
    }

    // Safe by removing the third level, 4.
    #[test]
    fn test_five() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_safe_report(&report));
    }

    // Safe without removing any level.
    #[test]
    fn test_six() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_safe_report(&report));
    }

    // Safe by removing first level.
    #[test]
    fn test_seven() {
        let report = vec![1, 1, 3, 6, 7, 9];
        assert!(is_safe_report(&report));
    }

    // Safe by removing last level.
    #[test]
    fn test_eight() {
        let report = vec![1, 2, 3, 6, 7, 7];
        assert!(is_safe_report(&report));
    }

    // Safe by removing last level.
    #[test]
    fn test_nine() {
        let report = vec![89, 90, 91, 94, 98];
        assert!(is_safe_report(&report));
    }

    // Safe by removing last level. Last level decreasing.
    #[test]
    fn test_ten() {
        let report = vec![89, 90, 91, 94, 0];
        assert!(is_safe_report(&report));
    }

    // Safe by removing first level. Order is changed.
    #[test]
    fn test_eleven() {
        let report = vec![99, 100, 97, 96, 95];
        assert!(is_safe_report(&report));
    }
}
