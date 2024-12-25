use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;

pub fn solve() {
    let mut file = File::open("./test_files/d2_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let safe_reports = buffer.lines()
        .map(|report| {
            report.split_ascii_whitespace()
                .map(|level| level.parse::<u8>().expect("Failed to parse int"))
                .collect::<Vec<u8>>()
        })
        .filter(|report| is_safe(&report, Ordering::Less) || is_safe(&report, Ordering::Greater))
        .count();
    
    println!("Safe Reports: {} ", safe_reports)
}

fn is_safe(report: &Vec<u8>, ordering: Ordering) -> bool {
    if report.len() < 3 {
        return true
    }

    let mut problems = 0;
    let mut prev = &report[0];
    let mut i = 1;

    if (prev.cmp(&report[1]) != ordering || prev.abs_diff(report[1]) > 3)
        && report[1].cmp(&report[2]) == ordering 
        && report[1].abs_diff(report[2]) <= 3 {
        prev = &report[2];
        problems = 1;
        i = 3;
    }
    
    for curr in &report[i..] {
        if prev.cmp(&curr) != ordering || prev.abs_diff(*curr) > 3 {
            problems += 1
        } else {
            prev = curr
        }
        if problems > 1 { 
            return false
        }
    }
    return true
}
