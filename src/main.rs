use std::fs;

mod day;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;

fn main() {
    let file = fs::read_to_string("input/day_11.txt").unwrap();
    println!("{}", day_11::part_01(file.as_str()));
    println!("{}", day_11::part_02(file.as_str()));
}
