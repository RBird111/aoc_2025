mod day_1;

fn main() {
    print_day(day_1::Day1);
}

pub trait Day {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

fn print_day<T: Day>(day: T) {
    let part_1 = day.part_1();
    println!("{part_1}");

    let part_2 = day.part_2();
    println!("{part_2}");
}
