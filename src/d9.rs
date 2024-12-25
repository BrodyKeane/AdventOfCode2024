use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut file = File::open("./test_files/d9_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let files = buffer.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect::<Vec<usize>>();
        
    let (mut file_id, mut free_space, mut file_size): (usize, usize, usize);
    let mut compressed: Vec<Option<usize>> = Vec::with_capacity(50000);

    for i in (0..files.len()).step_by(2) {
        file_id = i/2;
        file_size = files[i];
        free_space = match i+1 < files.len() {
            true => files[i+1],
            false => 0,
        };

        for _ in 0..file_size {
            compressed.push(Some(file_id));
        }

        for _ in 0..free_space {
            compressed.push(None);
        }
    }

    let mut i: usize;
    let mut j: usize = compressed.len() - 1;
    let mut file_size: usize;
    let mut free_space: usize;
    let mut file_id: Option<usize>;

    'outer: while j > 0 {
         if compressed[j].is_none() {
            j -= 1;
            continue
        }

        file_size = 0;
        file_id = compressed[j];
        while j > 0 && compressed[j] == file_id {
            j -= 1;
            file_size += 1;
        }

        free_space = 0;
        i = 0;
        while free_space < file_size {
            if i > j {
                continue 'outer;
            }
            match compressed[i] {
                Some(_) => free_space = 0,
                None => free_space += 1,
            }
            i += 1    
        }

        for k in 0..file_size {
            compressed[i - k - 1] = file_id;
            compressed[j + k + 1] = None;
        }
    }

    let mut checksum: usize = 0;
    for (i, file_id) in compressed.iter().enumerate() {
        if let Some(file_id) = file_id {
            checksum += i * file_id;
        }
    }

    for x in compressed {
        match x {
            Some(val) => print!("{}, ", val),
            None => print!("N, "),
        }
    }
    println!("");
    println!("Checksum: {}", checksum);
}
