use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Initialize vectors to store the numbers
    let mut vector1 = Vec::new();
    let mut vector2 = Vec::new();

    // Open the file
    if let Ok(file) = File::open("day1/part1/input.txt") {
        // Create a buffered reader
        let reader = io::BufReader::new(file);

        // Iterate over each line in the file
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                // Split the line by whitespace
                let parts: Vec<&str> = line.split_whitespace().collect();

                // Check if we have exactly two parts
                if parts.len() == 2 {
                    // Parse the numbers and push them to the respective vectors
                    if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        vector1.push(num1);
                        vector2.push(num2);
                    }
                }
            }
        });
    }

    assert_eq!(vector1.len(), vector2.len());
    // sort the vectors in ascending order
    vector1.sort();
    vector2.sort();
    let mut result = 0;
    for i in 0..vector1.len() {
        result += (vector1[i] - vector2[i]).abs();
    }
    println!("The result is: {}", result);

    Ok(())
}
