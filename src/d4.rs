use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut file = File::open("./test_files/d4_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let word_search = buffer.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut matches = 0;
    for (i, row) in word_search.iter().enumerate() {
        for (j, letter) in row.iter().enumerate() {
            if *letter == 'A' {
                if is_x_mas(&word_search, i, j) { 
                    matches += 1
                }
            }
        }
    }
    println!("{}", matches);
}

fn is_x_mas(word_search: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if  row < 1 ||
        col < 1 ||
        row >= word_search.len() - 1  ||
        col >= word_search[0].len() - 1 
    { 
        return false 
    }


    let lines = [
        [word_search[row-1][col-1], word_search[row+1][col+1]],
        [word_search[row-1][col+1], word_search[row+1][col-1]]
    ];

    for line in lines {
        if !line.contains(&'M') || !line.contains(&'S') {
            return false
        }
    }
    return true
}

fn xmas_search(word_search: &Vec<Vec<char>>, row: i32, col: i32) -> usize {
    let directions: [(i32, i32); 8] = [
        (-1, -1), // Top-left diagonal
        (-1,  0), // Up
        (-1,  1), // Top-right diagonal
        ( 0,  1), // Right
        ( 1,  1), // Bottom-right diagonal
        ( 1,  0), // Down
        ( 1, -1), // Bottom-left diagonal
        ( 0, -1), // Left
    ];

    let candidates: Vec<Vec<(i32, i32)>> = directions
        .iter()
        .filter_map(|&(di, dj)| {
            // Generate three steps in the direction (di, dj)
            let candidate: Vec<(i32, i32)> = (1..=3)
                .map(|step| (row + step * di, col + step * dj))
                .collect();

            let is_within_bounds = |row: i32, col: i32| {
                row >= 0 &&
                col >= 0 &&
                row < word_search.len() as i32 &&
                col < word_search[0].len() as i32
            };

            if candidate.iter().all(|&(x, y)| is_within_bounds(x, y)) {
                Some(candidate)
            } else {
                None
            }
        })
        .collect();


    let target = ['M', 'A', 'S'];
    let mut matches = 0;
    for candidate in candidates {
        for (i, (row, col)) in candidate.iter().enumerate() {
            if target[i] != word_search[*row as usize][*col as usize] {
                break;
            } else if i == 2 {
                matches += 1
            }
        }
    }
    return matches;
}
