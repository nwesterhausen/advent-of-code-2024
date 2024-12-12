use std::fs;

pub fn main() {
    println!("\nDay 2\n");

    let reports = read_input_file();
    let num_safe_reports = reports.iter().filter(|r| is_safe_report(r)).count();

    println!("Part 1 - Number of safe reports: {}", num_safe_reports);

    let unsafe_reports = reports.iter().filter(|r| !is_safe_report(r)).collect::<Vec<&Vec<i32>>>();
    let num_safe_reports_with_problem_dampener = unsafe_reports.iter().filter(|r| is_safe_report_with_problem_dampener(r)).count();
    
    println!("Part 2 - Number of safe reports with problem dampener: {}", num_safe_reports_with_problem_dampener);
    println!("Part 2 - Total number of 'safe' reports: {}", num_safe_reports + num_safe_reports_with_problem_dampener);
}

/// Check if the report passes with a problem dampener and tolerate a single invalid level
fn is_safe_report_with_problem_dampener(report: &Vec<i32>) -> bool {
    let (is_valid, problem_dampener) = is_homogenous_differences_with_problem_dampener(report);
    let mut problem_dampener = problem_dampener;
    if !is_valid {
        return false;
    }

    for i in 1..report.len() {
        // if our problem dampener is already used, we can't use it again. but we can use it once
        if !is_valid_difference(report[i - 1], report[i]) {
            if problem_dampener {
                return false;
            }
            problem_dampener = true;
        }
    }

    true
}

/// We can tolerate a single invalid level in a report.
/// 
/// Returns a tuple of two booleans:
/// - If the report is valid with a problem dampener
/// - If we had to use the problem dampener
fn is_homogenous_differences_with_problem_dampener(report: &Vec<i32>) -> (bool, bool) {
    let mut increasing = true;
    let mut decreasing = true;
    let mut problem_dampener = false;

    for i in 1..report.len() {
        if report[i] > report[i - 1] {
            decreasing = false;
        } else if report[i] < report[i - 1] {
            increasing = false;
        } else {
            if problem_dampener {
                return (false, true);
            }
            problem_dampener = true;
        }
    }

    (increasing || decreasing, problem_dampener)
}

/// Return true if the report is valid
fn is_safe_report(report: &Vec<i32>) -> bool {
    if !is_homogenous_differences(report) {
        return false;
    }

    for i in 1..report.len() {
        if !is_valid_difference(report[i - 1], report[i]) {
            return false;
        }
    }

    true
}

/// Check that the difference between two levels is within [1,3] inclusive
fn is_valid_difference(level1: i32, level2: i32) -> bool {
    (level1 - level2).abs() <= 3
}

/// Return true if the report is all increasing or all decreasing
fn is_homogenous_differences(report: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..report.len() {
        if report[i] > report[i - 1] {
            decreasing = false;
        } else if report[i] < report[i - 1] {
            increasing = false;
        } else {
            // the report is invalid if there are any duplicate values
            return false;
        }
    }

    increasing || decreasing
}

fn read_input_file() -> Vec<Vec<i32>> {
    // read the input file
    let file_contents = fs::read_to_string("inputs/day2.txt").expect("Unable to read input file!");

    let mut reports = Vec::new();

    for line in file_contents.lines() {
        // split each line by whitespace and create a Vec<i32>
        reports.push(
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect(),
        );
    }
    
    reports
}