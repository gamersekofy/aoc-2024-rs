//! `Part1` is a struct that provides functionality to read input from a file, 
//! process the data into two lists of integers, and calculate the sum of the 
//! absolute differences between corresponding elements in the two lists.
use std::fs;
use std::path::Path;

pub struct Part1;

impl Part1 {
    pub fn run(input_path: &str) {
        let file_contents = fs::read_to_string(Path::new(input_path)).expect("Unable to read file");

        let mut left_list: Vec<i32> = Vec::new();
        let mut right_list: Vec<i32> = Vec::new();

        for line in file_contents.lines() {
            let (left, right) = Self::parse_line(line);
            left_list.push(left);
            right_list.push(right);
        }

        println!(
            "The sum of the differences between the two lists is: {}",
            Self::calculate_diff(&mut left_list, &mut right_list)
        )
    }

    
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
