use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

pub fn solve() {
    let mut file = File::open("./test_files/d10_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let map: Vec<Vec<u8>> = buffer.lines()
        .map(|line| { line.chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as u8)
            .collect()
        })
        .collect();
    
    let mut trailheads: Vec<(usize, usize)> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                trailheads.push((row, col));
            }
        }
    }

    let mut unexplored: Vec<(usize, usize)>;
    //let mut explored: HashSet<(usize, usize)>;
    let mut accessible: Vec<(usize, usize)>;
    let mut summits_reached: usize;
    let mut score = 0;

    for (row, col) in trailheads {
        unexplored = vec![(row, col)];
        //explored = HashSet::new();
        summits_reached = 0;
        
        while !unexplored.is_empty() {
            let (row, col) = unexplored.pop().unwrap();
            if map[row][col] == 9 {
                summits_reached += 1;
                continue;
            }
            accessible = find_accessible(&map, row, col);
            unexplored.append(&mut accessible);
            //for location in accessible {
            //    if !explored.contains(&location) {
            //        unexplored.push(location);
            //        explored.insert(location);
            //    }
            //}
        }
        println!("Score: {}", summits_reached);
        score += summits_reached;
    }

    println!("Score: {}", score);
}
    
fn find_accessible(map: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<(usize, usize)> {
    let is_within_bounds = |row: i32, col: i32| {
        row >= 0 &&
        col >= 0 &&
        row < map.len() as i32 &&
        col < map[0].len() as i32
    };

    let directions: [(i32, i32); 4] = [
        (-1,  0), // Up
        ( 0,  1), // Right
        ( 1,  0), // Down
        ( 0, -1), // Left
    ];

    let (mut next_row, mut next_col): (i32, i32);
    let elevation: u8 = map[row][col];
    let mut accessible: Vec<(usize, usize)> = Vec::new();

    for (dx, dy) in directions {
        (next_row, next_col) = (dx + row as i32, dy + col as i32);
        if is_within_bounds(next_row, next_col) 
            && map[next_row as usize][next_col as usize] == elevation + 1 {
            accessible.push((next_row as usize, next_col as usize))
        }
    }
    return accessible
}


