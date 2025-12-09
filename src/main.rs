mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day_1::Day1),
        Box::new(day_2::Day2),
        Box::new(day_3::Day3),
        Box::new(day_4::Day4),
        Box::new(day_5::Day5),
        Box::new(day_6::Day6),
    ];
    days.into_iter().zip(1..).for_each(print_day);
}

pub trait Day {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

#[allow(clippy::boxed_local)]
fn print_day<T: Day + ?Sized>((day, id): (Box<T>, u32)) {
    println!("[DAY {}]:", id);
    println!("\tPART 1: {}", day.part_1());
    println!("\tPART 2: {}\n", day.part_2());
}
