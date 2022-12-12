
pub fn part_01(input: &str) -> String {
    // A = X = Rock, 1pt
    // B = Y = Paper, 2pt
    // C = Z = Scissor, 3pt
    // Loss = 0
    // Draw = 3
    // Win = 6
    input
        .lines()
        .map(|s| match s {
            "A X" => 1 + 3,
            "B X" => 1 + 0,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2 + 0,
            "A Z" => 3 + 0,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => panic!("Unknown string")
        })
        .sum::<i64>()
        .to_string()
}

pub fn part_02(input: &str) -> String {
    input
        .lines()
        .map(|s| match s {
            "A X" => 3 + 0,
            "B X" => 1 + 0,
            "C X" => 2 + 0,
            "A Y" => 1 + 3,
            "B Y" => 2 + 3,
            "C Y" => 3 + 3,
            "A Z" => 2 + 6,
            "B Z" => 3 + 6,
            "C Z" => 1 + 6,
            _ => panic!("Unknown string")
        })
        .sum::<i64>()
        .to_string()
}
