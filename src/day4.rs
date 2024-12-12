use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
fn part1(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'X' {
                if col + 3 < cols
                    && grid[row][col + 1] == 'M'
                    && grid[row][col + 2] == 'A'
                    && grid[row][col + 3] == 'S'
                {
                    count += 1;
                }

                if col >= 3
                    && grid[row][col - 1] == 'M'
                    && grid[row][col - 2] == 'A'
                    && grid[row][col - 3] == 'S'
                {
                    count += 1;
                }

                if row + 3 < rows
                    && grid[row + 1][col] == 'M'
                    && grid[row + 2][col] == 'A'
                    && grid[row + 3][col] == 'S'
                {
                    count += 1;
                }

                if row >= 3
                    && grid[row - 1][col] == 'M'
                    && grid[row - 2][col] == 'A'
                    && grid[row - 3][col] == 'S'
                {
                    count += 1;
                }

                if row + 3 < rows
                    && col + 3 < cols
                    && grid[row + 1][col + 1] == 'M'
                    && grid[row + 2][col + 2] == 'A'
                    && grid[row + 3][col + 3] == 'S'
                {
                    count += 1;
                }

                if row >= 3
                    && col >= 3
                    && grid[row - 1][col - 1] == 'M'
                    && grid[row - 2][col - 2] == 'A'
                    && grid[row - 3][col - 3] == 'S'
                {
                    count += 1;
                }

                if row + 3 < rows
                    && col >= 3
                    && grid[row + 1][col - 1] == 'M'
                    && grid[row + 2][col - 2] == 'A'
                    && grid[row + 3][col - 3] == 'S'
                {
                    count += 1;
                }

                if row >= 3
                    && col + 3 < cols
                    && grid[row - 1][col + 1] == 'M'
                    && grid[row - 2][col + 2] == 'A'
                    && grid[row - 3][col + 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_valid_diagonal(a: char, b: char) -> bool {
    (a == 'S' && b == 'M') || (a == 'M' && b == 'S')
}

#[aoc(day4, part2)]
fn part2(grid: &[Vec<char>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] == 'A' {
                let top_left = grid[row - 1][col - 1];
                let top_right = grid[row - 1][col + 1];
                let bottom_left = grid[row + 1][col - 1];
                let bottom_right = grid[row + 1][col + 1];

                if is_valid_diagonal(top_left, bottom_right)
                    && is_valid_diagonal(top_right, bottom_left)
                {
                    count += 1;
                }
            }
        }
    }

    count
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
    fn part2_sample() {
        let input = std::fs::read_to_string("input/2024/sample4.txt").expect("Failed to read file");
        assert_eq!(part2(&parse(&input)), 9);
    }

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("input/2024/day4.txt").expect("Failed to read file");
        assert_eq!(part2(&parse(&input)), 1890);
    }
}
