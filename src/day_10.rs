
struct Computer {
    cycle: usize,
    x: isize,
    strengths: Vec<isize>,
    sprite: Vec<char>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            cycle: 0,
            x: 1,
            strengths: Vec::new(),
            sprite: vec!['░'; 240],
        }
    }

    fn run_cycle(&mut self) {
        println!("c: {}, x: {}, drawing: {}", self.cycle, self.x, if self.x - 1 <= self.cycle as isize && self.cycle as isize <= self.x + 1 { "#" } else { "." });

        if self.x - 1 <= self.cycle as isize && self.cycle as isize <= self.x + 1 {
            self.sprite[self.cycle] = '█';
        }

        self.cycle += 1;
        if self.cycle % 40 == 0 {
            self.x += 40;
        }
    }

    fn execute(&mut self, line: &str) {
        self.strengths.push(self.x);
        self.run_cycle();

        if line.starts_with("addx") {
            self.strengths.push(self.x);
            self.run_cycle();
            self.x += line[5..].parse::<isize>().unwrap();
        }
    }
}

pub fn part_01(input: &str) -> String {
    let mut c = Computer::new();

    input
        .lines()
        .for_each(|l| c.execute(l));

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        // .inspect(|v| println!("{:3}: {:4} * {:3} = {}", v, c.strengths[v-1], v, c.strengths[v-1] * *v as isize))
        .map(|v| c.strengths[v-1] * v as isize)
        .sum::<isize>()
        .to_string()
}

pub fn part_02(input: &str) -> String {
    let mut c = Computer::new();

    input
        .lines()
        .for_each(|l| c.execute(l));

    println!("{:?}", c.sprite);

    [40, 80, 120, 160, 200, 240]
        .into_iter()
        .map(|v| c.sprite[(v-40)..v].iter().map(|c| c.to_string() + &c.to_string()).collect::<String>() + "\n")
        .collect()
}
