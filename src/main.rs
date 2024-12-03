mod day1;
mod day2;

fn print_day(day: i32) {
    print!("---------- Day {} -----------\n", day);
}

fn main() {
    print_day(1);
    day1::day1();

    print_day(2);
    day2::day2();
}
