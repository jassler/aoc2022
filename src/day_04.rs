use std::cmp;
use regex::Regex;

pub fn part_01(input: &str) -> String {
    let r = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    input
        .lines()
        .map(|l| {
            let cap = r.captures(l).unwrap();
            let l1 = cap[1].parse::<i64>().unwrap();
            let l2 = cap[2].parse::<i64>().unwrap();
            let r1 = cap[3].parse::<i64>().unwrap();
            let r2 = cap[4].parse::<i64>().unwrap();

            if (r1 <= l1 && l2 <= r2) ||
                (l1 <= r1 && r2 <= l2) {
                1
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}

pub fn part_02(input: &str) -> String {
    let r = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    input
        .lines()
        .map(|l| {
            let cap = r.captures(l).unwrap();
            let l1 = cap[1].parse::<i64>().unwrap();
            let l2 = cap[2].parse::<i64>().unwrap();
            let r1 = cap[3].parse::<i64>().unwrap();
            let r2 = cap[4].parse::<i64>().unwrap();

            if (r1 <= l1 && r2 >= l1) ||
                (l1 <= r1 && l2 >= r1) {
                1
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}
