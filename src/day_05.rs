//             [G] [W]         [Q]
// [Z]         [Q] [M]     [J] [F]
// [V]         [V] [S] [F] [N] [R]
// [T]         [F] [C] [H] [F] [W] [P]
// [B] [L]     [L] [J] [C] [V] [D] [V]
// [J] [V] [F] [N] [T] [T] [C] [Z] [W]
// [G] [R] [Q] [H] [Q] [W] [Z] [G] [B]
// [R] [J] [S] [Z] [R] [S] [D] [L] [J]
//  1   2   3   4   5   6   7   8   9

use regex::Regex;

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let m = stacks
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // transpose characters and do some filtering
    (0..m[m.len()-1].len())
        .map(|i| m
            .iter()
            .map(|inner| if i < inner.len() { inner[i] } else { ' ' })
            .filter(|c| *c != ' ')
            .rev()
            .skip(1)
            .collect::<Vec<char>>()
        )
        .filter(|v| v.len() > 0 && ('A' <= v[0] && v[0] <= 'Z'))
        .collect()
}

pub fn part_01(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut stacks = parse_stacks(parts[0]);
    stacks.insert(0, vec![]);


    // move 6 from 5 to 7
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    parts[1]
        .lines()
        .for_each(|l| {
            let captures = re.captures(l).unwrap();
            let mv: usize = captures[1].parse().unwrap();
            let fr: usize = captures[2].parse().unwrap();
            let to: usize = captures[3].parse().unwrap();
            for _ in 0..mv {
                let c = stacks[fr].pop().unwrap();
                stacks[to].push(c);
            }
        });

    stacks
        .iter()
        .skip(1)
        .map(|v| v[v.len()-1])
        .collect()
}

pub fn part_02(input: &str) -> String {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut stacks = parse_stacks(parts[0]);
    stacks.insert(0, vec![]);


    // move 6 from 5 to 7
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    parts[1]
        .lines()
        .for_each(|l| {
            let captures = re.captures(l).unwrap();
            let mv: usize = captures[1].parse().unwrap();
            let fr: usize = captures[2].parse().unwrap();
            let to: usize = captures[3].parse().unwrap();
            let s = stacks[fr].len();

            let mut slice: Vec<char> = stacks[fr].drain(s-mv..s).collect();
            stacks[to].append(&mut slice);

            // this doesn't work (signed, borrow checker)
            // let mut slice = stacks[fr].drain(s-mv..s);
            // stacks[to].extend(&mut slice);
        });

    stacks
        .iter()
        .skip(1)
        .map(|v| v[v.len()-1])
        .collect()
}
