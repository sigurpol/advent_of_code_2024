use std::fs::File;
use std::io::{self, BufRead};

fn read_file_to_char_matrix(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Create the matrix
    let mut matrix = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        matrix.push(row);
    }

    Ok(matrix)
}

fn count_x_pattern_occurrences(matrix: &[Vec<char>]) -> usize {
    let n = matrix.len();
    let m = if n > 0 { matrix[0].len() } else { 0 };

    let mut count = 0;

    for i in 1..n - 1 {
        for j in 1..m - 1 {
            // Check the 'X' pattern centered at (i, j)
            if is_x_pattern(matrix, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn is_x_pattern(matrix: &[Vec<char>], x: usize, y: usize) -> bool {
    let patterns = [
        // Forward MAS
        [('M', 'A', 'S'), ('M', 'A', 'S')],
        // Backward SAM
        [('S', 'A', 'M'), ('S', 'A', 'M')],
        // Mixed patterns
        [('M', 'A', 'S'), ('S', 'A', 'M')],
        [('S', 'A', 'M'), ('M', 'A', 'S')],
    ];

    for pattern in &patterns {
        let ((top_left, center, bottom_right), (top_right, _, bottom_left)) =
            (pattern[0], pattern[1]);

        if matrix[x - 1][y - 1] == top_left
            && matrix[x][y] == center
            && matrix[x + 1][y + 1] == bottom_right
            && matrix[x - 1][y + 1] == top_right
            && matrix[x + 1][y - 1] == bottom_left
        {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let file_path = "day04/part2/input.txt";

    match read_file_to_char_matrix(file_path) {
        Ok(matrix) => {
            println!(
                "Number of XMAS occurrences: {}",
                count_x_pattern_occurrences(&matrix)
            );
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    Ok(())
}
