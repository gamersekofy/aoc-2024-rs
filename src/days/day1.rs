use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn run(input_path: &str) {
    let file_contents = fs::read_to_string(Path::new(input_path)).expect("Unable to read file");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in file_contents.lines() {
        let (left, right) = Part1::parse_line(line);
        left_list.push(left);
        right_list.push(right);
    }

    println!(
        "Part\tResult\n\
    ----\t------\n\
    1\t{}\n\
    2\t{}",
        Part1::calculate_diff(&mut left_list, &mut right_list)
            .to_string()
            .green(),
        Part2::calculate_similarity_score(left_list, right_list)
            .to_string()
            .green()
    );
}

pub struct Part1;
pub struct Part2;

impl Part1 {
    /// Parses a line of text into a tuple of two integers.
    ///
    /// The function takes a string slice `line` as input, splits it into whitespace-separated parts,
    /// and parses the first two parts as integers. It is expected that the input string contains
    /// exactly two whitespace-separated values that can be parsed as 32-bit signed integers (`i32`).
    /// If the input string does not meet these expectations, the function will panic.
    ///
    /// # Arguments
    ///
    /// * `line` - A string slice containing two whitespace-separated values to be parsed as integers.
    ///
    /// # Returns
    ///
    /// A tuple containing two `i32` integers parsed from the input string.
    ///
    /// # Panics
    ///
    /// This function will panic in the following scenarios:
    /// - If the input string contains fewer than two parts when split by whitespace.
    /// - If either of the first two parts cannot be parsed into an `i32`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let line = "42 78";
    /// let result = parse_line(line);
    /// assert_eq!(result, (42, 78));
    /// ```
    ///
    /// ```rust
    /// let line = "-10 2048";
    /// let result = parse_line(line);
    /// assert_eq!(result, (-10, 2048));
    /// ```
    fn parse_line(line: &str) -> (i32, i32) {
        let mut parts = line.split_whitespace();
        let left = parts.next().unwrap().parse::<i32>().unwrap();
        let right = parts.next().unwrap().parse::<i32>().unwrap();
        (left, right)
    }

    /// Calculates the absolute difference between corresponding elements of two sorted lists
    /// and returns the sum of these differences.
    ///
    /// # Parameters:
    /// - `left_list`: A mutable reference to a vector of integers.
    ///   This vector will be sorted in-place during the function execution.
    /// - `right_list`: A mutable reference to a vector of integers.
    ///   This vector will also be sorted in-place during the function execution.
    ///
    /// # Returns:
    /// - An integer representing the sum of absolute differences between the corresponding
    ///   elements of the two sorted lists.
    ///
    /// # Panics:
    /// - The function will panic if the lengths of `left_list` and `right_list` are not equal
    ///   because it assumes that both vectors have the same length for element-wise comparison.
    ///
    /// # Example:
    /// ```
    /// let mut left = vec![3, 1, 4];
    /// let mut right = vec![1, 5, 9];
    /// let result = calculate_diff(&mut left, &mut right);
    /// assert_eq!(result, 10); // |1-1| + |3-5| + |4-9| = 0 + 2 + 5 = 10
    /// ```
    fn calculate_diff(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) -> i32 {
        let mut sum: i32 = 0;

        left_list.sort();
        right_list.sort();

        for i in 0..left_list.len() {
            sum = sum + (left_list[i] - right_list[i]).abs();
        }
        sum
    }
}

impl Part2 {
    /// Computes a similarity score between two lists of integers.
    ///
    /// This function calculates a score by taking the sum of each element in the `left_list`
    /// multiplied by the number of times it appears in the `right_list`.
    ///
    /// # Arguments
    ///
    /// * `left_list` - A vector of integers representing the first list.
    /// * `right_list` - A vector of integers representing the second list.
    ///
    /// # Returns
    ///
    /// * An `i64` value representing the similarity score between the two lists. If there are
    ///   no common elements between the lists, the score will be `0`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let left_list = vec![1, 2, 3, 2];
    /// let right_list = vec![2, 3, 3];
    /// let score = calculate_similarity_score(left_list, right_list);
    /// assert_eq!(score, 11); // (2 * 2) + (3 * 3) = 4 + 9 = 11
    /// ```
    ///
    /// # How it works
    ///
    /// 1. A frequency map (`HashMap`) is created for all elements in `right_list`, where the keys are the
    ///    values of the elements and the values are the counts.
    /// 2. For each element in `left_list`, we check if it exists in the frequency map:
    ///    - If it exists, its value is multiplied by its count in `right_list`.
    ///    - If it does not exist, it contributes `0` to the sum.
    /// 3. The result is the total sum of these products.
    ///
    /// # Complexity
    ///
    /// - Time Complexity: O(n + m), where `n` is the length of `right_list` and `m` is the
    ///   length of `left_list`.
    /// - Space Complexity: O(n), for storing the frequency map of `right_list`.
    fn calculate_similarity_score(left_list: Vec<i32>, right_list: Vec<i32>) -> i64 {
        let right_counts: HashMap<i32, i64> =
            right_list.iter().fold(HashMap::new(), |mut map, &num| {
                *map.entry(num).or_insert(0) += 1;
                map
            });

        left_list
            .iter()
            .map(|&num| {
                let right_count = right_counts.get(&num).copied().unwrap_or(0);
                (num as i64) * right_count
            })
            .sum()
    }
}
