use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    // The input have two sections.
    // The first section specifies the page ordering rules, one per line. The first rule, 47|53, means that if an update includes both page number 47 and page number 53, then page number 47 must be printed at some point before page number 53.
    // The second section specifies the page numbers of each update. Because most safety manuals are different, the pages needed in the updates are different too. The first update, 75,47,61,53,29, means that the update consists of page numbers 75, 47, 61, 53, and 29.

    // Implement a code that read the input to retrieve the first and store them into a list
    // For example: 47|53, 97|13, 97|61 becomes {47 -> 53, 97 -> 13, 61}

    // First, let's split the input into two sections: rules and update pages.
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules_section = parts[0];
    let updates_section = parts[1];
    // Parse rules and store them into a dictionary (key, list of values).
    let mut rules_before: std::collections::HashMap<u32, Vec<u32>> =
        std::collections::HashMap::new();
    let mut rules_after: std::collections::HashMap<u32, Vec<u32>> =
        std::collections::HashMap::new();
    for rule in rules_section.lines() {
        let numbers: Vec<u32> = rule.split("|").map(|s| s.trim().parse().unwrap()).collect();
        let first = numbers[0];
        let second = numbers[1];
        rules_before
            .entry(first)
            .or_insert_with(Vec::new)
            .push(second);
        rules_after
            .entry(second)
            .or_insert_with(Vec::new)
            .push(first);
    }
    // Parse update pages and store them into a vector for each line.
    let updates: Vec<Vec<u32>> = updates_section
        .lines()
        .map(|s| s.split(",").map(|s| s.trim().parse().unwrap()).collect())
        .collect();

    println!("{:?}", rules_before);
    println!("{:?}", rules_after);
    println!("{:?}", updates);
    todo!()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u32 {
    todo!()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u32 {
    todo!()
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
        assert_eq!(part1(&parse("<EXAMPLE>")), u32::MAX);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), u32::MAX);
    }
}
