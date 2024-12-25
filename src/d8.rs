use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let mut file = File::open("./test_files/d8_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let map: Vec<Vec<char>> = buffer.lines()
        .map(|line| line.chars().collect())
        .collect();
    

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '.' {
                continue
            }
            antennas.entry(map[row][col])
                .or_insert(Vec::new())
                .push((row, col))
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for locations in antennas.values() {
        for (i, (row1, col1)) in locations.iter().enumerate() {
            for (row2, col2) in locations[i+1..].iter() {
                let new_nodes = find_antinodes(
                    *row1 as isize,
                    *col1 as isize,
                    *row2 as isize,
                    *col2 as isize,
                    map.len() as isize,
                    map[0].len() as isize
                );
                antinodes.extend(new_nodes)
            }
        }
    }

    println!("# of antinodes: {}", antinodes.len());
}

fn find_antinodes(row1: isize, col1: isize, row2: isize, col2: isize, row_bounds: isize, col_bounds: isize
    ) -> HashSet<(isize, isize)> {
    let mut antinodes = HashSet::new();

    let is_within_bounds = |row: isize, col: isize| {
        row >= 0
        && col >= 0
        && row < row_bounds
        && col < col_bounds
    };

    let dx = row2 - row1;
    let dy = col2 - col1;
    let (mut row3, mut col3) = (row1, col1);

    while is_within_bounds(row3, col3) {
        antinodes.insert((row3, col3));
        row3 -= dx;
        col3 -= dy;
    }

    let (mut row3, mut col3) = (row1, col1);
    while is_within_bounds(row3, col3) {
        antinodes.insert((row3, col3));
        row3 += dx;
        col3 += dy;
    }
    
    return antinodes
}
