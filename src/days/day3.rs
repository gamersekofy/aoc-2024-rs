/*
Notes:
    - Puzzle output is corrupted memory of computer
    - Goal of program is to multiply numbers
    - Valid call of mul(x,y) function is just that: mul(x,y)
    - Scan input for all valid mul calls and, execute them, and add them up
 */
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn run(input_file_path: &str) {
    let memory = Part1::parse_memory_file(input_file_path);
    let mut total = 0;
    for mem_line in memory {
        total += Part1::eval_mem_line(mem_line);
    }
    println!("The total is: {}", total);
}

pub struct Part1;

impl Part1 {
    fn parse_memory_file(input_file_path: &str) -> Vec<String> {
        let mut memory_vec: Vec<String> = Vec::new();
        let file_contents = fs::read_to_string(Path::new(input_file_path)).unwrap();
        for line in file_contents.lines() {
            memory_vec.push(line.to_string());
        }
        memory_vec
    }

    fn eval_mem_line(line: String) -> u32 {
        let regex = Regex::new(r"mul\((\d+),\s*(-?\d+)\)").unwrap();
        let mut total_product_for_line = 0;

        for cap in regex.captures_iter(&line) {
            let num1_str = cap.get(1).unwrap().as_str();
            let num2_str = cap.get(2).unwrap().as_str();

            if let (Ok(num1), Ok(num2)) = (num1_str.parse::<u32>(), num2_str.parse::<u32>()) {
                total_product_for_line += (num1 * num2);
            }
        }
        total_product_for_line
    }
}
