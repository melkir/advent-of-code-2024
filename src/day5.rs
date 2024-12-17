use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Rules = HashMap<(u32, u32), bool>;
type Update = Vec<u32>;
type Input = (Rules, Vec<Update>);

struct PageOrderValidator<'a> {
    rules: &'a Rules,
}

impl<'a> PageOrderValidator<'a> {
    fn new(rules: &'a Rules) -> Self {
        Self { rules }
    }

    fn is_valid_order(&self, update: &[u32]) -> bool {
        for (i, &earlier_page) in update.iter().enumerate() {
            for &later_page in update.iter().skip(i + 1) {
                if self.rules.contains_key(&(later_page, earlier_page)) {
                    return false;
                }
            }
        }
        true
    }

    fn sort_update(&self, update: &[u32]) -> Vec<u32> {
        let mut sorted = update.to_vec();
        sorted.sort_by(|&a, &b| {
            match (
                self.rules.contains_key(&(a, b)),
                self.rules.contains_key(&(b, a)),
            ) {
                (true, _) => std::cmp::Ordering::Less,
                (_, true) => std::cmp::Ordering::Greater,
                _ => b.cmp(&a),
            }
        });
        sorted
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let (rules_section, updates_section) = input.split_once("\n\n").expect("Invalid input format");

    let updates = updates_section
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().parse().unwrap()).collect())
        .collect();

    let rules = rules_section
        .lines()
        .map(|rule| {
            let mut nums = rule.split('|').map(|s| s.trim().parse().unwrap());
            let before = nums.next().unwrap();
            let after = nums.next().unwrap();
            ((before, after), true)
        })
        .collect();

    (rules, updates)
}

#[aoc(day5, part1)]
fn part1((rules, updates): &Input) -> u32 {
    let validator = PageOrderValidator::new(rules);

    updates
        .iter()
        .filter(|update| validator.is_valid_order(update))
        .map(|update| update[update.len() / 2])
        .sum()
}
#[aoc(day5, part2)]
fn part2((rules, updates): &Input) -> u32 {
    let validator = PageOrderValidator::new(rules);

    updates
        .iter()
        .filter(|update| !validator.is_valid_order(update))
        .map(|update| {
            let sorted = validator.sort_update(update);
            sorted[sorted.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = std::fs::read_to_string("input/2024/sample5.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 143);
    }

    #[test]
    fn part1_example() {
        let input = std::fs::read_to_string("input/2024/day5.txt").expect("Failed to read file");
        assert_eq!(part1(&parse(&input)), 4766);
    }

    #[test]
    fn part2_sample() {
        let input = std::fs::read_to_string("input/2024/sample5.txt").expect("Failed to read file");
        assert_eq!(part2(&parse(&input)), 123);
    }

    #[test]
    fn part2_example() {
        let input = std::fs::read_to_string("input/2024/day5.txt").expect("Failed to read file");
        assert_eq!(part2(&parse(&input)), 6257);
    }
}
