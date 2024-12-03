use regex::Regex;
use std::fs;
use std::io::{self};

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

fn calculate_mul_sum(input: &str) -> i32 {
    let mut result = 0;
    let mut mul_enabled = true; // Initially, mul instructions are enabled

    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut last_pos = 0;
    while last_pos < input.len() {
        if let Some(mul_match) = mul_re.find(&input[last_pos..]) {
            let mul_start = last_pos + mul_match.start();
            let mul_end = last_pos + mul_match.end();

            let do_match = do_re.find(&input[last_pos..mul_start]);
            let dont_match = dont_re.find(&input[last_pos..mul_start]);

            if let Some(do_pos) = do_match {
                if dont_match.is_none() || do_pos.start() > dont_match.unwrap().start() {
                    mul_enabled = true;
                }
            }

            if let Some(dont_pos) = dont_match {
                if do_match.is_none() || dont_pos.start() > do_match.unwrap().start() {
                    mul_enabled = false;
                }
            }

            if mul_enabled {
                if let Some(captures) = mul_re.captures(&input[mul_start..mul_end]) {
                    let x: i32 = captures[1].parse().unwrap();
                    let y: i32 = captures[2].parse().unwrap();
                    result += x * y;
                }
            }

            last_pos = mul_end;
        } else {
            break;
        }
    }

    result
}

fn main() -> io::Result<()> {
    let file_path = "day3/part2/input.txt";
    match read_file_to_string(file_path) {
        Ok(contents) => println!("Result:{}", calculate_mul_sum(&contents)),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    Ok(())
}
