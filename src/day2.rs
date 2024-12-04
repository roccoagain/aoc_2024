use std::fs;

pub fn day2() {
    // open the input ./inputs/day2.txt
    let input = fs::read_to_string("./inputs/day2.txt").unwrap();

    // split by lines
    let lines: Vec<&str> = input.lines().collect();

    // initialize counters for part 1 and part 2
    let mut safe_reports_part1 = 0;
    let mut safe_reports_part2 = 0;

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // function to check if a sequence is safe
        let is_safe = |seq: &[i32]| -> bool {
            if seq.len() < 2 {
                return false; // not enough data to determine monotonicity
            }

            // check if the sequence is monotonic increasing or decreasing
            let is_increasing = seq.windows(2).all(|w| w[1] >= w[0]);
            let is_decreasing = seq.windows(2).all(|w| w[1] <= w[0]);

            if !is_increasing && !is_decreasing {
                return false; // sequence is not monotonic
            }

            // check differences between adjacent numbers
            seq.windows(2).all(|w| {
                let diff = (w[1] - w[0]).abs();
                diff >= 1 && diff <= 3
            })
        };

        // part 1: check if the original sequence is safe
        if is_safe(&numbers) {
            safe_reports_part1 += 1;
            safe_reports_part2 += 1; // also counts for part 2
            continue;
        }

        // part 2: try removing each level one at a time
        for i in 0..numbers.len() {
            let mut modified_numbers = numbers.clone();
            modified_numbers.remove(i);

            if is_safe(&modified_numbers) {
                safe_reports_part2 += 1;
                break; // no need to check further if we've found a safe sequence
            }
        }

        // if no safe sequence is found after removing one level, the report remains unsafe
    }

    println!("safe reports (part 1): {}", safe_reports_part1);
    assert!(safe_reports_part1 == 421);

    println!("safe reports (part 2): {}", safe_reports_part2);
    assert!(safe_reports_part2 == 476);
}
