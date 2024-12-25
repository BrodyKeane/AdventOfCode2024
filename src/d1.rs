use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub fn solve() {
    let mut file = File::open("./test_files/d1-test1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let mut ids1: Vec<u32>= Vec::new();
    let mut ids2: HashMap<u32, u32> = HashMap::new();

    for (i, id) in buffer.split_ascii_whitespace().enumerate() {
        let id = id.parse::<u32>().expect(&format!("Failed to Parse int: '{}'", id));
        match i % 2 {
            0 => ids1.push(id),
            1 => *ids2.entry(id).or_insert(0) += 1,
            _ => ()
        }
    }
    
    println!("{:?}", ids1);
    println!("{:?}", ids2);

    let mut sum = 0;
    for id in ids1 {
        sum += id * ids2.get(&id).unwrap_or(&0);
    }
        
    println!("{}", sum);
}


