mod tools;

fn main() {
    let reports = tools::read_reports_from_stdin().expect("Failed to read lines");
    let safe_reports = count_safe_reports(reports);
    println!("{}", safe_reports.len());
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    reports.into_iter().filter(is_safe_report).collect()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let mut level_prev: i32 = report[0];
    for level in report.iter().skip(1) {
        if *level == level_prev {
            return false;
        }
        if (level_prev - level).abs() > 3 {
            return false;
        }
        level_prev = *level;
    }

    report.is_sorted() || report.iter().rev().is_sorted()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Safe because the levels are all decreasing by 1 or 2.
    #[test]
    fn test_is_safe_report_sorted_ascending() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_safe_report(&report));
    }

    // Unsafe because 2 7 is an increase of 5.
    #[test]
    fn test_is_safe_report_sorted_descending() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_safe_report(&report));
    }

    // Unsafe because 6 2 is a decrease of 4.
    #[test]
    fn test_is_safe_report_unsorted() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_safe_report(&report));
    }

    // Unsafe because 1 3 is increasing but 3 2 is decreasing.
    #[test]
    fn test_is_safe_report_empty() {
        let report: Vec<i32> = vec![1, 3, 2, 4, 5];
        assert!(!is_safe_report(&report));
    }

    // Unsafe because 4 4 is neither an increase or a decrease.
    #[test]
    fn test_is_safe_report_single_element() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(!is_safe_report(&report));
    }

    // Safe because the levels are all increasing by 1, 2, or 3.
    #[test]
    fn test_is_safe_report_single_element2() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_safe_report(&report));
    }
}
