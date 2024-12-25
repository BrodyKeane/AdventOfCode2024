use std::fs::File;
use std::io::Read;

struct Guard {
    row: isize,
    col: isize,
    direction: Direction,
}

impl Guard {
    pub fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn next_pos(&self) -> (isize, isize) {
        return match self.direction {
            Direction::North => (self.row - 1, self.col),
            Direction::East => (self.row, self.col + 1),
            Direction::South => (self.row + 1, self.col),
            Direction::West => (self.row, self.col - 1),
        }
    }

    pub fn move_to(&mut self, row: isize, col: isize) {
        self.row = row;
        self.col = col;
    }
}

enum Direction {
    North,
    East,
    South,
    West
}

pub fn solve() {
    let mut file = File::open("./test_files/d6_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let mut map: Vec<Vec<char>> = 
        buffer.lines()
            .map(|line| line.chars().collect())
            .collect();

    let mut guard: Option<Guard> = None;
    for (i, row) in map.iter().enumerate() {
        if let Some(j) = row.iter().position(|&tile| tile == '^') {
            map[i][j] = 'X';
            guard = Some(Guard{ row: i as isize, col: j as isize, direction: Direction::North });
            break
        }
    }
    let guard = guard.unwrap();

    let tiles_moved = track_guard(guard, map);
    println!("{}", tiles_moved)
}

fn track_guard(mut guard: Guard, mut map: Vec<Vec<char>>) -> usize {
    let map_height = map.len();
    let map_width = map[0].len();
    let within_bounds = |row: isize, col: isize| {
        row >= 0 &&
        col >= 0 &&
        row < map_height as isize &&
        col < map_width as isize
    };
    
    let (mut next_row, mut next_col): (isize, isize);
    let mut tiles_covered = 1;
    while within_bounds(guard.row, guard.col) {
        (next_row, next_col) = guard.next_pos();
        if !within_bounds(next_row, next_col) {
            break
        }

        match map[next_row as usize][next_col as usize] {
            '.' => {
                guard.move_to(next_row, next_col);
                map[next_row as usize][next_col as usize] = 'X';
                tiles_covered += 1;
            },
            'X' => guard.move_to(next_row, next_col),
            '#' => guard.turn(),
            unknown => panic!("Unknown symbol found: {}", unknown),
        } 
    }

    return tiles_covered
}
