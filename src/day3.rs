use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .split(')')
        .filter_map(|s| {
            if let Some(mul_part) = s.split("mul(").last() {
                let nums: Vec<u32> = mul_part.split(',').filter_map(|n| n.parse().ok()).collect();
                if nums.len() == 2 {
                    Some((nums[0], nums[1]))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<(u32, u32)>) -> u32 {
    todo!()
}

#[aoc(day3, part2)]
fn part2(input: &Vec<(u32, u32)>) -> u32 {
    todo!()
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
    fn part1_example() {
        let input = std::fs::read_to_string("input/2024/day3.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
