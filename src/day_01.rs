use std::cmp;

pub fn part_01(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part_02(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<u64>>();
    result.sort();
    let s = result.len();
    (result[s-1] + result[s-2] + result[s-3]).to_string()
}
