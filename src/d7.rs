use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut file = File::open("./test_files/d7_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let total: u64 = buffer
        .lines()
        .map(|line| {
            let mut groups = line.split(":");
            let target: u64 = groups.next()
                .expect("No colon found in line")
                .parse()
                .expect("No int found before ':'");
            let values: Vec<u64> = groups.next()
                .expect("No values found after ':'")
                .split_whitespace()
                .filter_map(|value| {
                    value.parse::<u64>()
                        .ok()
                })
                .collect();
            match is_valid_equation(target, values) {
                true => target,
                false => 0,
            }
        })
        .sum();

    println!("Total: {}", total);
}

fn is_valid_equation(target: u64, values: Vec<u64>) -> bool {
    let mut total = values[0];
    let mut operations: Vec<Operation> = vec!();
    let mut i = 1;

    loop {
        if i == values.len() {
            if  total == target {
                return true
            }

            if !backtrack(&mut total, &values, &mut operations, &mut i) {
                return false
            }
            continue
        }

        if total * values[i] <= target {
            total *= values[i];
            operations.push(Operation::Multiply);
            i += 1;
        } else if total + values[i] <= target {
            total += values[i];
            operations.push(Operation::Add);
            i += 1;
        } else if !backtrack(&mut total, &values, &mut operations, &mut i) {
            return false
        }
    }
}

fn backtrack(total: &mut u64, values: &Vec<u64>, operations: &mut Vec<Operation>, i: &mut usize) -> bool {
    *i -= 1;
    loop {
        let op = match operations.pop() {
            Some(op) => op,
            None => return false,
        };
        match op {
            Operation::Add => {
                *total -= values[*i];
                *i -= 1
            },
            Operation::Multiply => {
                *total /= values[*i];
                *total += values[*i];
                operations.push(Operation::Add);
                *i += 1;
                return true
            },
        }
    }
}


#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}








