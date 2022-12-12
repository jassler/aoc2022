fn contains_duplicates(w: &[char]) -> bool {
    for i in 0..(w.len()-1) {
        for j in (i+1)..w.len() {
            if w[i] == w[j] {
                return true;
            }
        }
    }
    return false;
}

pub fn part_01(input: &str) -> String {
    let v: Vec<char> = input.chars().collect();
    let windows = v.windows(4);

    let mut count = 4;
    for w in windows {
        if contains_duplicates(w) {
            count += 1;
        } else {
            break;
        }
    }

    count.to_string()
}

pub fn part_02(input: &str) -> String {
    let v: Vec<char> = input.chars().collect();
    let windows = v.windows(14);

    let mut count = 14;
    for w in windows {
        if contains_duplicates(w) {
            count += 1;
        } else {
            break;
        }
    }

    count.to_string()
}
