use std::fs;
use std::io::{self};

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

fn calculate_mul_sum(input: &str) -> i32 {
    let mut result = 0;

    // Define a regular expression to match valid "mul(X,Y)" patterns
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Iterate over all matches in the input string
    for caps in re.captures_iter(input) {
        // Extract the numbers X and Y from the capture groups
        if let (Some(x_match), Some(y_match)) = (caps.get(1), caps.get(2)) {
            // Parse the captured strings as integers
            if let (Ok(x), Ok(y)) = (
                x_match.as_str().parse::<i32>(),
                y_match.as_str().parse::<i32>(),
            ) {
                // Add the product to the result
                result += x * y;
            }
        }
    }

    result
}

fn main() -> io::Result<()> {
    let file_path = "day03/part1/input.txt";
    match read_file_to_string(file_path) {
        Ok(contents) => println!("Result:{}", calculate_mul_sum(&contents)),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mul_sum_empty() {
        let input = "";
        assert_eq!(calculate_mul_sum(input), 0);
    }

    #[test]
    fn test_calculate_mul_sum_single() {
        let input = "mul(2,3)";
        assert_eq!(calculate_mul_sum(input), 6);
    }

    #[test]
    fn test_calculate_mul_sum_multiple() {
        let input = "mul(2,3) mul(4,5)";
        assert_eq!(calculate_mul_sum(input), 26);
    }

    #[test]
    fn test_calculate_mul_sum_invalid() {
        let input = "mul(2,3) mul(a,b)";
        assert_eq!(calculate_mul_sum(input), 6);
    }

    #[test]
    fn test_calculate_mul_sum_mixed() {
        let input = "mul(2,3) some text mul(4,5)";
        assert_eq!(calculate_mul_sum(input), 26);
    }
}
