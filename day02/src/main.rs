use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_input(filename: &str) -> std::io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        // println!("{:?}", report);
        reports.push(report);
    }

    Ok(reports)
}

fn safe_report(report: Vec<i32>) -> bool {
    let mut last_diff;
    let mut current_diff = 0;
    let mut safe = true;

    // println!("REPORT");
    for (i, _level) in report.iter().enumerate() {
        last_diff = current_diff;
        // skip first number because compairing pairs
        if i != 0 {
            current_diff = report[i] - report[i-1];
            // println!("Level: {}, Diff: {}-{}={}, {}", level, report[i], report[i-1], current_diff, last_diff);
            if (current_diff.abs() < 1) || (current_diff.abs() > 3) {
                // println!("Not Safe: Current diff = {}", current_diff);
                safe = false;
                break;
            }
            if (current_diff * last_diff) < 0 { 
                // println!("Not Safe: Last diff = {}, Current diff = {}", last_diff, current_diff);
                safe = false;
                break;
            }
        }
    }
    
    safe
}

fn part1(reports: &Vec<Vec<i32>>) {
    let mut total_safe = 0;

    for report in reports {
        if safe_report(report.to_vec()) { total_safe += 1; }
    }

    println!("Total Safe: {}", total_safe);
}

fn part2(reports: &mut Vec<Vec<i32>>) {
    let mut total_safe = 0;

    for report in reports {
        if safe_report(report.to_vec()) { total_safe += 1; }
        else {
            // println!("Unsafe Report: {:?}", report);
            let mut modified_report: Vec<i32>;
            for i in 0..report.len() {
                modified_report = report.clone();
                modified_report.remove(i);
                // println!("Modified Report: {:?}", modified_report);
                if safe_report(modified_report) {
                    total_safe += 1;
                    break;
                }
            }
        }
    }

    println!("Total Safe with Problem Dampener: {}", total_safe);
}

fn main() {
    let filename = "input.txt";
    let Ok(mut reports) = read_input(filename) else {return};
    part1(&reports);
    part2(&mut reports);
}
