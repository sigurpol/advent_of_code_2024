use std::fs::File;
use std::io::{self, BufRead};

// Define a method to check if a Vec<i32> is safe.
// A vector is safe if:
// 1. elements are all in ascending or descending order
// 2. delta between elements is at min 1 an at max 3
fn is_safe(v: &[i32]) -> bool {
    if v.is_empty() {
        return true; // not sure here
    }

    // Check if the vector is sorted in ascending or descending order
    let is_ascending = v.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = v.windows(2).all(|w| w[0] >= w[1]);

    if !is_ascending && !is_descending {
        return false;
    }

    // Check the delta between elements
    if !v.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0]).abs())) {
        return false;
    }
    true
}

fn main() -> io::Result<()> {
    let mut vectors: Vec<Vec<i32>> = Vec::new();
    // Open the file
    if let Ok(file) = File::open("day2/part1/input.txt") {
        // Create a buffered reader
        let reader = io::BufReader::new(file);

        // Iterate over each line in the file
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                // Split the line by whitespace
                let parts: Vec<&str> = line.split_whitespace().collect();
                // Parse the parts into integers and add them to the vectors
                vectors.push(parts.iter().map(|s| s.parse::<i32>().unwrap()).collect());
            }
        });
    }

    let mut result = 0;
    for v in &vectors {
        if is_safe(v) {
            result += 1;
        }
    }

    println!("The result is: {}", result);

    Ok(())
}
