use std::fs;

pub fn day1() {
    // read input file
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();

    // split input by newlines
    let lines: Vec<&str> = input.lines().collect();

    // there are two numbers per line. put all the left numbers in one array and all the right numbers in another array
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    // sort both from least to greatest
    left.sort();
    right.sort();

    // part 1
    // sum how far each left number is from the corresponding right number
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    println!("Distance Score: {}", sum);

    // part 2
    // find how often each number from the left list appears in the right list
    let mut count = 0;
    for &num in &left {
        let occurrences = right.iter().filter(|&&x| x == num).count();
        count += num * occurrences as i32;
    }
    println!("Similarity Score: {}", count);
}
