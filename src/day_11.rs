use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Monkey {
    items: VecDeque<usize>,
    inspection_count: usize,
    op_char: char,
    op_operand: String,
    div_by: usize,
    if_true: usize,
    if_false: usize,
}

struct MonkeyResult {
    new_item: usize,
    throw_to_monkey: usize,
}

impl Monkey {
    fn determine_recipient(&self, val: usize) -> MonkeyResult {
        MonkeyResult {
            new_item: val,
            throw_to_monkey: if val % self.div_by == 0 { self.if_true } else { self.if_false },
        }
    }

    fn inspect_item(&mut self) -> Option<MonkeyResult> {
        if let Some(it) = self.items.pop_front() {
            self.inspection_count += 1;
            let rhs = if self.op_operand == "old" { it } else { self.op_operand.parse().unwrap() };
            Some(match &self.op_char {
                '+' => self.determine_recipient(((it + rhs) / 3)),
                '*' => self.determine_recipient(((it * rhs) / 3)),
                _ => panic!("Shouldn't be here"),
            })
        } else {
            None
        }
    }

    fn inspect_item_2(&mut self, modu: usize) -> Option<MonkeyResult> {
        if let Some(it) = self.items.pop_front() {
            self.inspection_count += 1;
            let rhs = if self.op_operand == "old" { it } else { self.op_operand.parse().unwrap() };
            Some(match &self.op_char {
                '+' => self.determine_recipient((it + rhs) % modu),
                '*' => self.determine_recipient((it * rhs) % modu),
                _ => panic!("Shouldn't be here"),
            })
        } else {
            None
        }
    }
}

fn parse_monkey(lines: &str) -> Monkey {
    println!("{lines}\n\n");
    let lines = lines.split('\n').collect::<Vec<&str>>();
    let items = &lines[1]["  Starting items: ".len()..];
    let op = &lines[2]["  Operation: new = old ".len()..];

    Monkey {
        items: VecDeque::from_iter(items.split(", ").map(|i| i.parse::<usize>().unwrap())),
        inspection_count: 0,
        op_char: op.chars().nth(0).unwrap(),
        op_operand: op[2..].to_string(),
        div_by: lines[3]["  Test: divisible by ".len()..].parse().unwrap(),
        if_true: lines[4]["    If true: throw to monkey ".len()..].parse().unwrap(),
        if_false: lines[5]["    If false: throw to monkey ".len()..].parse().unwrap(),
    }
}

fn simulate_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].inspect_item() {
            monkeys[item.throw_to_monkey].items.push_back(item.new_item);
        }
    }
}

pub fn part_01(input: &str) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| parse_monkey(s))
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        simulate_round(&mut monkeys);
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    (monkeys[0].inspection_count * monkeys[1].inspection_count).to_string()
}

fn simulate_round_2(monkeys: &mut Vec<Monkey>, modu: usize) {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].inspect_item_2(modu) {
            monkeys[item.throw_to_monkey].items.push_back(item.new_item);
        }
    }
}


pub fn part_02(input: &str) -> String {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| parse_monkey(s))
        .collect::<Vec<Monkey>>();

    let mut modu = 1;
    monkeys.iter().for_each(|m| modu *= m.div_by);

    for _ in 0..10000 {
        simulate_round_2(&mut monkeys, modu);
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    (monkeys[0].inspection_count * monkeys[1].inspection_count).to_string()
}
