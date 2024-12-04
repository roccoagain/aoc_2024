mod solutions;

fn print_header(day: i32) {
    print!("---------- Day {} -----------\n", day);
}

fn main() {
    print_header(1);
    solutions::day1::day1();

    print_header(2);
    solutions::day2::day2();

    print_header(3);
    solutions::day3::day3();

    print_header(4);
    solutions::day4::day4();
}
