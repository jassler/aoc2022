
pub fn part_01(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_at(l.len() / 2);
            let r = r.chars().collect::<Vec<char>>();

            for c in l.chars() {
                if r.contains(&c) {
                    return if c >= 'a' {
                        c as u64 - 'a' as u64 + 1
                    } else {
                        c as u64 - 'A' as u64 + 27
                    }
                }
            }
            panic!("Shouldn't be here.");
        })
        .sum::<u64>()
        .to_string()
}

pub fn part_02(input: &str) -> String {
    let mut it = input.lines().peekable();
    let mut sum = 0;

    while it.peek().is_some() {
        let mut res: Vec<char> = it.next().unwrap().chars().collect();
        let sec: Vec<char> = it.next().unwrap().chars().collect();
        let thi: Vec<char> = it.next().unwrap().chars().collect();

        res.retain(|x| sec.contains(x) && thi.contains(x));
        if res.len() == 0 {
            println!("Res: {:?}", res);
            println!("Sec: {:?}", sec);
            println!("Thi: {:?}", thi);
            panic!("Wrong amount of letters left.");
        }
        let c = res[0];
        sum += if c >= 'a' {
            c as u64 - 'a' as u64 + 1
        } else {
            c as u64 - 'A' as u64 + 27
        }
    }

    sum.to_string()
}
