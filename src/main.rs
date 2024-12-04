mod day1;
mod day2;
mod day3;
mod day4;

fn print_header(day: i32) {
    print!("---------- Day {} -----------\n", day);
}

fn main() {
    print_header(1);
    day1::day1();

    print_header(2);
    day2::day2();

    print_header(3);
    day3::day3();

    print_header(4);
    day4::day4();
}
