use std::fs::File;
use std::io::{self, BufRead};

// Define a method to check if a Vec<i32> is safe.
// A vector is safe if:
// 1. elements are all in ascending or descending order
// 2. delta between elements is at min 1 an at max 3
// 3. if we remove an element from a vector and that makes the vector safe, the vector itself is
//    marked as safe.
fn is_safe(v: &Vec<i32>) -> bool {
    // Helper function to check if a vector is safe according to rules 1 and 2
    fn check_safety(v: &[i32]) -> bool {
        let is_ascending = v.windows(2).all(|w| w[0] <= w[1]);
        let is_descending = v.windows(2).all(|w| w[0] >= w[1]);

        if !is_ascending && !is_descending {
            return false;
        }

        v.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0]).abs()))
    }

    // Check if the vector itself is safe
    if check_safety(v) {
        return true;
    }

    // Check if removing one element makes the vector safe
    for i in 0..v.len() {
        let mut modified = v.clone();
        modified.remove(i);
        if check_safety(&modified) {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let mut vectors: Vec<Vec<i32>> = Vec::new();
    // Open the file
    if let Ok(file) = File::open("day2/part2/input.txt") {
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
