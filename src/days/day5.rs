use colored::Colorize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

pub fn run(input_path: &str) {
    let file_contents = fs::read_to_string(Path::new(input_path)).expect("Unable to read file");

    let (rules, updates) = parse_input(&file_contents);

    println!(
        "Part\tResult\n\
        ----\t------\n\
        1\t{}\n\
        2\t{}",
        Part1::solve(&rules, &updates).to_string().green(),
        Part2::solve(&rules, &updates).to_string().green()
    );
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    for line in parts[0].lines() {
        let rule_parts: Vec<&str> = line.split('|').collect();
        let before = rule_parts[0].parse::<i32>().unwrap();
        let after = rule_parts[1].parse::<i32>().unwrap();

        rules
            .entry(before)
            .or_insert_with(HashSet::new)
            .insert(after);
    }

    let updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn is_correctly_ordered(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for i in 0..update.len() {
        for j in (i + 1)..update.len() {
            let current_page = update[i];
            let later_page = update[j];

            if let Some(pages_after) = rules.get(&later_page) {
                if pages_after.contains(&current_page) {
                    return false;
                }
            }
        }
    }
    true
}

pub struct Part1;
pub struct Part2;

impl Part1 {
    /// Solves part 1 of the Print Queue problem.
    ///
    /// Finds all correctly-ordered updates and returns the sum of their middle page numbers.
    ///
    /// # Arguments
    ///
    /// * `rules` - A HashMap mapping page numbers to sets of pages that must come after them
    /// * `updates` - A vector of update sequences, each containing page numbers to be printed
    ///
    /// # Returns
    ///
    /// The sum of middle page numbers from all correctly-ordered updates
    fn solve(rules: &HashMap<i32, HashSet<i32>>, updates: &[Vec<i32>]) -> i32 {
        let mut sum = 0;

        for update in updates {
            if is_correctly_ordered(update, rules) {
                let middle_index = update.len() / 2;
                sum += update[middle_index];
            }
        }
        sum
    }
}

impl Part2 {
    /// Solves part 2 of the Print Queue problem.
    ///
    /// Finds all incorrectly-ordered updates, fixes their order according to the rules,
    /// and returns the sum of their middle page numbers after correction.
    ///
    /// # Arguments
    ///
    /// * `rules` - A HashMap mapping page numbers to sets of pages that must come after them
    /// * `updates` - A vector of update sequences, each containing page numbers to be printed
    ///
    /// # Returns
    ///
    /// The sum of middle page numbers from all incorrectly-ordered updates after fixing their order
    fn solve(rules: &HashMap<i32, HashSet<i32>>, updates: &[Vec<i32>]) -> i32 {
        let mut sum = 0;

        for update in updates {
            if !is_correctly_ordered(update, rules) {
                let corrected_update = Self::fix_order(update.clone(), rules);
                let middle_index = corrected_update.len() / 2;
                sum += corrected_update[middle_index];
            }
        }
        sum
    }

    /// Fixes the order of an incorrectly-ordered update according to the page ordering rules.
    ///
    /// Uses a bubble sort approach where adjacent elements are swapped if they violate
    /// the ordering rules. The process continues until no more swaps are needed.
    ///
    /// # Arguments
    ///
    /// * `update` - A vector of page numbers in incorrect order
    /// * `rules` - A HashMap mapping page numbers to sets of pages that must come after them
    ///
    /// # Returns
    ///
    /// A vector of page numbers in the correct order according to the rules
    ///
    /// # Example
    ///
    /// If the rules indicate that 97 must come before 75, and we have [75, 97, 47],
    /// this function will return [97, 75, 47].
    fn fix_order(mut update: Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
        let len = update.len();

        for i in 0..len {
            for j in 0..(len - i - 1) {
                let current = update[j];
                let next = update[j + 1];

                // Check if current should come after next according to the rules
                if let Some(pages_after) = rules.get(&next) {
                    if pages_after.contains(&current) {
                        // Swap them - next should come before current
                        update.swap(j, j + 1);
                    }
                }
            }
        }

        update
    }
}
