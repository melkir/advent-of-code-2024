use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<(i32, i32)> {
    let mut left_nums = Vec::new();
    let mut right_nums = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left_nums.push(nums[0]);
        right_nums.push(nums[1]);
    }

    // Sort both vectors
    left_nums.sort();
    right_nums.sort();

    // Pair up numbers in order
    let pairs: Vec<(i32, i32)> = left_nums.into_iter().zip(right_nums.into_iter()).collect();

    pairs
}

#[aoc(day1, part1, i32)]
fn part1(input: &Vec<(i32, i32)>) -> i32 {
    let mut distance = 0;
    for (x, y) in input {
        distance += (x - y).abs();
    }

    distance
}

#[aoc(day1, part2, i32)]
fn part2(input: &Vec<(i32, i32)>) -> i32 {
    let mut number_map: HashMap<i32, i32> = HashMap::new();

    for (_, y) in input {
        *number_map.entry(*y).or_insert(0) += 1;
    }

    let mut score = 0;
    for (x, _) in input {
        if number_map.contains_key(&x) {
            score += x * number_map[&x];
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&parse(TEST_INPUT)), 11);
    }

    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("input/2024/day1.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 2430334);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&parse(TEST_INPUT)), 31);
    }

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("input/2024/day1.txt").expect("Failed to read file");
        assert_eq!(part2(&parse(&input)), 2430334);
    }
}
