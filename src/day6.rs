// use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use std::{fmt, fs};

struct Guard {
    x: usize,
    y: usize,
    facing: Direction,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_map = self.map.clone(); // Clone the map to modify it

        // Replace the guard's position with the character from its direction
        let guard_char = self.guard.facing.to_char();
        display_map[self.guard.y][self.guard.x] = guard_char;

        for row in display_map.iter() {
            let line: String = row.iter().collect(); // Convert Vec<char> to String
            writeln!(f, "{}", line)?; // Write the line to the formatter
        }
        Ok(())
    }
}

impl Guard {
    fn new(x: usize, y: usize, facing: Direction) -> Self {
        Guard { x, y, facing }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn forward(&mut self) {
        let (x, y) = self.get_next_move();
        if x < 0 || y < 0 {
            return;
        }
        self.x = x as usize;
        self.y = y as usize;
    }

    fn get_next_move(&self) -> (isize, isize) {
        let (dx, dy) = match self.facing {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        (self.x as isize + dx, self.y as isize + dy)
    }
}

struct Grid {
    guard: Guard,
    // visited: HashSet<(usize, usize)>,
    map: Vec<Vec<char>>, // 2D map representation
}

impl Grid {
    fn new(guard: Guard, map: Vec<Vec<char>>) -> Self {
        Grid {
            guard,
            // visited: HashSet::new(),
            map,
        }
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<Cell> {
        if x < 0 || y < 0 || x as usize >= self.map[0].len() || y as usize >= self.map.len() {
            return None; // Out of bounds
        }
        Some(self.map[y as usize][x as usize].into())
    }

    fn patrol(&mut self) {
        loop {
            let (x, y) = (self.guard.x, self.guard.y);
            let (next_x, next_y) = self.guard.get_next_move();

            // println!("{}\n\n", self);

            if let Some(cell) = self.get_cell(next_x, next_y) {
                match cell {
                    Cell::Empty => {
                        // Mark the current position as visited
                        self.map[y][x] = 'X';
                        // self.visited.insert((x, y));
                        self.guard.forward()
                    }
                    Cell::Obstacle => self.guard.turn_right(),
                    Cell::Guard => panic!("Guard is facing itself"),
                }
            } else {
                self.map[y][x] = 'X';
                break;
            }
        }
    }

    #[allow(unused)]
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut display_map = self.map.clone(); // Clone the map to modify it
        let guard_char = self.guard.facing.to_char();
        display_map[self.guard.y][self.guard.x] = guard_char;

        let grid_string: String = display_map
            .iter()
            .map(|row| row.iter().collect::<String>()) // Convert each row to a String
            .collect::<Vec<String>>() // Collect into a Vec<String>
            .join("\n"); // Join rows with newlines

        fs::write(filename, grid_string) // Write to the specified file
    }
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Cell {
    Guard,
    Obstacle,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '^' | '<' | '>' | 'v' => Cell::Guard,
            '#' => Cell::Obstacle,
            '.' | 'X' => Cell::Empty,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    // Find the position of the guard represented by '^'
    let (mut guard_x, mut guard_y) = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        if let Some(x) = line.find('^') {
            guard_x = x;
            guard_y = y;
            break; // Stop after finding the first occurrence
        }
    }

    // Initialize the guard with the found coordinates
    let guard = Guard::new(guard_x, guard_y, Direction::Up);

    let mut grid = Grid::new(guard, map);
    grid.patrol();
    grid
}

#[aoc(day6, part1)]
fn part1(grid: &Grid) -> u32 {
    let count_x = grid
        .map
        .iter()
        .flat_map(|row| row.iter()) // Flatten the rows into a single iterator
        .filter(|&&cell| cell == 'X') // Filter for 'X' characters
        .count();

    count_x as u32
}

#[aoc(day6, part2)]
fn part2(grid: &Grid) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = std::fs::read_to_string("input/2024/sample6.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 41);
    }

    #[test]
    fn part1_example() {
        let input: String =
            std::fs::read_to_string("input/2024/day6.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 4988);
    }

    #[test]
    fn part2_example() {
        let input: String =
            std::fs::read_to_string("input/2024/day6.txt").expect("Failed to read file");
        assert_eq!(part2(&parse(&input)), u32::MAX);
    }
}
