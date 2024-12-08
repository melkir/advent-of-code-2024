use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> String {
    input.to_string()
}

fn parse_levels(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_safe(levels: &[u32]) -> bool {
    if levels.len() < 2 {
        return true;
    }
    let inc = levels.windows(2).all(|p| p[0] < p[1]);
    let dec = levels.windows(2).all(|p| p[0] > p[1]);
    let diff_adj = levels.windows(2).all(|p| p[0].abs_diff(p[1]) <= 3);
    (inc || dec) && diff_adj
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let levels = parse_levels(line);
            is_safe(&levels)
        })
        .count() as u32
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| is_safe_with_dampener(line))
        .count() as u32
}

fn is_safe_with_dampener(report: &str) -> bool {
    let levels: Vec<u32> = parse_levels(report);

    if is_safe(&levels) {
        return true;
    }

    // Check if the Problem Dampener can make the report safe
    for i in 0..levels.len() {
        let mut dampened_levels = levels.clone();
        dampened_levels.remove(i);
        if is_safe(&dampened_levels) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&parse(TEST_INPUT)), 2);
    }

    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("input/2024/day2.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 572);
    }

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("input/2024/day2.txt").expect("Failed to read file");
        assert_eq!(part2(&parse(&input)), 612);
    }
}
