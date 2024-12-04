use std::fs;

pub fn day4() {
    // read the input file ./inputs/day4.txt
    let input = fs::read_to_string("./inputs/day4.txt").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    let n_rows = lines.len();
    let n_cols = if lines.is_empty() { 0 } else { lines[0].len() };

    // check horizontal "XMAS" or "SAMX"
    for row in &lines {
        for col in 0..row.len().saturating_sub(3) {
            let word: String = row[col..col + 4].iter().collect();
            if word == "XMAS" || word == "SAMX" {
                sum += 1;
            }
        }
    }

    // check vertical "XMAS" or "SAMX"
    for col in 0..n_cols {
        for row in 0..n_rows.saturating_sub(3) {
            let word: String = (0..4).map(|k| lines[row + k][col]).collect();
            if word == "XMAS" || word == "SAMX" {
                sum += 1;
            }
        }
    }

    // check diagonal down-right "XMAS" or "SAMX"
    for row in 0..n_rows.saturating_sub(3) {
        for col in 0..n_cols.saturating_sub(3) {
            let word: String = (0..4).map(|k| lines[row + k][col + k]).collect();
            if word == "XMAS" || word == "SAMX" {
                sum += 1;
            }
        }
    }

    // check diagonal down-left "XMAS" or "SAMX"
    for row in 0..n_rows.saturating_sub(3) {
        for col in 3..n_cols {
            let word: String = (0..4).map(|k| lines[row + k][col - k]).collect();
            if word == "XMAS" || word == "SAMX" {
                sum += 1;
            }
        }
    }

    println!("Sum of all operations: {}", sum);
    assert!(sum == 2532);

    // part 2
    let mut sum2 = 0;

    // loop over the grid to find "X-MAS" patterns
    for row in 1..n_rows.saturating_sub(1) {
        for col in 1..n_cols.saturating_sub(1) {
            // ensure indices are within bounds
            if row >= 1 && row + 1 < n_rows && col >= 1 && col + 1 < n_cols {
                // first diagonal: positions (row-1,col-1), (row,col), (row+1,col+1)
                let word1: String = vec![
                    lines[row - 1][col - 1],
                    lines[row][col],
                    lines[row + 1][col + 1],
                ]
                .into_iter()
                .collect();

                // second diagonal: positions (row-1,col+1), (row,col), (row+1,col-1)
                let word2: String = vec![
                    lines[row - 1][col + 1],
                    lines[row][col],
                    lines[row + 1][col - 1],
                ]
                .into_iter()
                .collect();

                let valid_words = ["MAS", "SAM"];

                if valid_words.contains(&word1.as_str()) && valid_words.contains(&word2.as_str()) {
                    sum2 += 1;
                }
            }
        }
    }

    println!("Sum of all X-MAS patterns: {}", sum2);
    assert!(sum2 == 1941);
}
