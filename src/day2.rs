use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> String {
    input.to_string()
}

fn is_valid(line: &str) -> bool {
    // For each line we have multiple levels splitted by a whitespace
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    let levels = line
        .split_whitespace()
        .map(|s| str::parse::<u32>(s).unwrap())
        .collect::<Vec<_>>();
    let inc = levels.windows(2).all(|p| p[0] < p[1]);
    let dec = levels.windows(2).all(|p| p[0] > p[1]);
    let diff_adj = levels.windows(2).all(|p| p[0].abs_diff(p[1]) <= 3);
    (inc || dec) && diff_adj
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let val = is_valid(&line);
        if val == true {
            count += 1;
        }
    }
    count
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    todo!()
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
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
