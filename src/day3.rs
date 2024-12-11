use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3, part1)]
fn parse(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .filter_map(|cap| Some((cap[1].parse().ok()?, cap[2].parse().ok()?)))
        .collect()
}

#[aoc_generator(day3, part2)]
fn parse_instruction(input: &str) -> Vec<(u32, u32)> {
    todo!()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<(u32, u32)>) -> u32 {
    let sum: u32 = input.iter().map(|(a, b)| a * b).sum();
    sum
}

#[aoc(day3, part2)]
fn part2(input: &Vec<(u32, u32)>) -> u32 {
    let sum: u32 = input.iter().map(|(a, b)| a * b).sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn parse_sample() {
        let extracted: Vec<(u32, u32)> = parse(TEST_INPUT);
        assert_eq!(extracted, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(part1(&parse(&TEST_INPUT)), 161);
    }

    #[test]
    fn part1_example_parse() {
        let input = std::fs::read_to_string("input/2024/day3.txt").expect("Failed to read file");
        assert_eq!(parse(&input).len(), 692);
    }

    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("input/2024/day3.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 175615763);
    }

    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn parse_part2_sample() {
        let extracted: Vec<(u32, u32)> = parse_instruction(TEST_INPUT2);
        assert_eq!(extracted, vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(&parse_instruction(&TEST_INPUT2)), 48);
    }

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("input/2024/day3.txt").expect("Failed to read file");
        assert_eq!(part2(&parse_instruction(&input)), 0);
    }
}
