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

fn count_xmas_occurrences(matrix: &[Vec<char>]) -> usize {
    let word = "XMAS";
    let directions = [
        (0, 1),  // Right
        (1, 0),  // Down
        (1, 1),  // Down-right
        (1, -1), // Down-left
    ];
    let n = matrix.len();
    let m = if n > 0 { matrix[0].len() } else { 0 };
    let word_chars: Vec<char> = word.chars().collect();
    let word_chars_reverse: Vec<char> = word_chars.iter().rev().cloned().collect();

    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            for &(dx, dy) in &directions {
                // Check in the forward direction
                if check_word(matrix, i, j, dx, dy, &word_chars) {
                    count += 1;
                }
                // Check in the reverse direction
                if check_word(matrix, i, j, dx, dy, &word_chars_reverse) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_word(
    matrix: &[Vec<char>],
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    word: &[char],
) -> bool {
    let n = matrix.len();
    let m = if n > 0 { matrix[0].len() } else { 0 };
    let word_len = word.len();

    for k in 0..word_len {
        let nx = x as isize + k as isize * dx;
        let ny = y as isize + k as isize * dy;

        if nx < 0 || ny < 0 || nx >= n as isize || ny >= m as isize {
            return false;
        }

        if matrix[nx as usize][ny as usize] != word[k] {
            return false;
        }
    }

    true
}

fn main() -> io::Result<()> {
    let file_path = "day04/part1/input.txt";

    match read_file_to_char_matrix(file_path) {
        Ok(matrix) => {
            println!(
                "Number of XMAS occurrences: {}",
                count_xmas_occurrences(&matrix)
            );
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    Ok(())
}
