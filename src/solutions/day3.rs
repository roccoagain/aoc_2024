use regex::Regex;
use std::fs;

pub fn day3() {
    // read the input file
    let input = fs::read_to_string("./inputs/day3.txt").unwrap();

    // define a pattern for "mul(x,y)"
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // part 1
    let mut sum = 0;
    for line in input.lines() {
        for cap in mul_regex.captures_iter(line) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            sum += x * y;
        }
    }
    println!("Sum of all operations: {}", sum);
    assert!(sum == 184122457);

    // define regex patterns
    let instruction_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // part 2
    let mut sum = 0;
    let mut enabled = true;
    for line in input.lines() {
        for mat in instruction_regex.find_iter(line) {
            let instr = &line[mat.start()..mat.end()];
            if instr.contains("do()") {
                enabled = true;
            } else if instr.contains("don't()") {
                enabled = false;
            } else if let Some(cap) = mul_regex.captures(instr) {
                if enabled {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    sum += x * y;
                }
            }
        }
    }
    println!("Sum of all enabled operations: {}", sum);
    assert!(sum == 107862689);
}
