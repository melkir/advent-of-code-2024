use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day5)]
fn parse(input: &str) -> (HashMap<(u32, u32), bool>, Vec<Vec<u32>>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules_section = parts[0];
    let updates_section = parts[1];

    // Parse updates
    let updates = updates_section
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().parse().unwrap()).collect())
        .collect();

    let mut rules: HashMap<(u32, u32), bool> = HashMap::new();
    for rule in rules_section.lines() {
        let numbers: Vec<u32> = rule.split('|').map(|s| s.trim().parse().unwrap()).collect();
        let (before, after) = (numbers[0], numbers[1]);
        rules.insert((before, after), true);
    }

    (rules, updates)
}

// Function to check if an update is in correct order
fn is_valid_order(update: &[u32], rules: &HashMap<(u32, u32), bool>) -> bool {
    // For each pair of pages in the update
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let earlier_page = update[i];
            let later_page = update[j];

            // If there's a rule saying later_page should come before earlier_page,
            // this order is invalid
            if rules.contains_key(&(later_page, earlier_page)) {
                return false;
            }
        }
    }
    true
}

// Function to sort an update according to rules
fn sort_update(update: &[u32], rules: &HashMap<(u32, u32), bool>) -> Vec<u32> {
    let mut sorted = update.to_vec();
    sorted.sort_by(|&a, &b| {
        if rules.contains_key(&(a, b)) {
            std::cmp::Ordering::Less
        } else if rules.contains_key(&(b, a)) {
            std::cmp::Ordering::Greater
        } else {
            b.cmp(&a) // Default to descending order if no rule exists
        }
    });
    sorted
}

#[aoc(day5, part1)]
fn part1(input: &(HashMap<(u32, u32), bool>, Vec<Vec<u32>>)) -> u32 {
    let (rules, updates) = input;

    let mut sum = 0;
    for update in updates {
        if is_valid_order(update, &rules) {
            sum += update[update.len() / 2];
        }
    }
    sum
}

#[aoc(day5, part2)]
fn part2(input: &(HashMap<(u32, u32), bool>, Vec<Vec<u32>>)) -> u32 {
    let (rules, updates) = input;

    let mut sum = 0;
    for update in updates {
        if !is_valid_order(update, &rules) {
            let sorted = sort_update(update, &rules);
            sum += sorted[sorted.len() / 2];
        }
    }
    sum
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
