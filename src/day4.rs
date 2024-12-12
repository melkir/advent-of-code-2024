use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn check_direction(grid: &[Vec<char>], row: i32, col: i32, row_dir: i32, col_dir: i32) -> bool {
    let word = "XMAS";
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (i, expected_char) in word.chars().enumerate() {
        let new_row = row + (i as i32 * row_dir);
        let new_col = col + (i as i32 * col_dir);

        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        if grid[new_row as usize][new_col as usize] != expected_char {
            return false;
        }
    }
    true
}

#[aoc(day4, part1)]
fn part1(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Define all possible directions (including diagonals)
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(row_dir, col_dir) in &directions {
                if check_direction(grid, row as i32, col as i32, row_dir, col_dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<char>]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = std::fs::read_to_string("input/2024/sample4.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 18);
    }

    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("input/2024/day4.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 2493);
    }

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("input/2024/day4.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), u32::MAX);
    }
}
