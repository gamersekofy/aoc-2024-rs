use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub struct Part1;

impl Part1 {
    pub fn run(file_path: &str) -> i32 {
        let input = fs::read_to_string(file_path).expect("Failed to read input file");
        Self::process_print_queue(&input)
    }

    fn process_print_queue(input: &str) -> i32 {
        let (rules, updates) = Self::parse_input(input);
        let mut sum = 0;

        for update in updates {
            if Self::is_correctly_ordered(&update, &rules) {
                let middle_index = update.len() / 2;
                sum += update[middle_index];
            }
        }
        sum
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
}
