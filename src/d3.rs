use std::fs::File;
use std::io::Read;

use regex::Regex;

pub fn solve() {
    let mut file = File::open("./test_files/d3_input1.txt").expect("Failed to open file");
    let mut buffer = String::from("do()");
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let conditional_regex = Regex::new(r"do\(\)(.*?)(?:don't\(\)|\n)").unwrap();
    let multiply_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result: usize = conditional_regex.captures_iter(&buffer)
        .map(|c| {
            let (_, [do_block]) = c.extract();
            println!("{}", do_block);
            multiply_regex.captures_iter(do_block)
                .map(|c| {
                    let (_, [int1, int2]) = c.extract();
                    int1.parse::<usize>().unwrap() * int2.parse::<usize>().unwrap()
                })
        })
        .flatten()
        .sum();

    println!("{}", result) 
}
