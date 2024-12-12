use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day4)]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    todo!()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
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
        assert_eq!(part1(&parse(&input)), u32::MAX);
    }

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("input/2024/day4.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), u32::MAX);
    }
}
