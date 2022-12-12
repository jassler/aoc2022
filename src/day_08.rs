use std::cmp::max;

#[derive(Debug)]
struct Tree {
    height: isize,
    max_left: isize,
    max_right: isize,
    max_top: isize,
    max_bottom: isize,
}

impl Tree {
    fn new(height: isize) -> Tree {
        Tree {
            height,
            max_left: -1,
            max_right: -1,
            max_top: -1,
            max_bottom: -1,
        }
    }
}

fn init_tree_heights(trees: &mut Vec<Vec<Tree>>) {
    for y in 1..(trees.len()-1) {
        for x in 1..(trees[y].len()-1) {
            trees[y][x].max_left = max(trees[y][x-1].max_left, trees[y][x-1].height);
            trees[y][x].max_top = max(trees[y-1][x].max_top, trees[y-1][x].height);
        }
    }

    for y in (1..(trees.len()-1)).rev() {
        for x in (1..(trees[y].len()-1)).rev() {
            trees[y][x].max_right = max(trees[y][x+1].max_right, trees[y][x+1].height);
            trees[y][x].max_bottom = max(trees[y+1][x].max_bottom, trees[y+1][x].height);
        }
    }
}

fn init_trees(input: &str) -> Vec<Vec<Tree>> {
    let mut trees: Vec<Vec<Tree>> = Vec::new();

    for line in input.lines() {
        let mut v = Vec::new();
        line.chars().for_each(|c| v.push(Tree::new(c as isize - '0' as isize)));
        trees.push(v);
    }

    init_tree_heights(&mut trees);
    trees
}

pub fn part_01(input: &str) -> String {
    let trees = init_trees(input);

    trees
        .iter()
        .flatten()
        .into_iter()
        .filter(|t| t.height > t.max_left || t.height > t.max_right || t.height > t.max_top || t.height > t.max_bottom)
        .count()
        .to_string()
}

fn get_tree_score(trees: &Vec<Vec<Tree>>, y: usize, x: usize) -> usize {
    let mut result = 1;
    let current_height = trees[y][x].height;

    let mut i = y - 1;
    while i > 0 && trees[i][x].height < current_height { i -= 1; }
    result *= y - i;

    i = y + 1;
    while i < (trees.len()-1) && trees[i][x].height < current_height { i += 1; }
    result *= i - y;


    i = x - 1;
    while i > 0 && trees[y][i].height < current_height { i -= 1; }
    result *= x - i;

    i = x + 1;
    while i < (trees[y].len()-1) && trees[y][i].height < current_height { i += 1; }

    result * (i - x)
}

pub fn part_02(input: &str) -> String {
    let trees = init_trees(input);
    let mut max_score = 0;

    for y in 1..(trees.len()-1) {
        for x in 1..(trees[y].len()-1) {
            max_score = max(max_score, get_tree_score(&trees, y, x));
        }
    }

    max_score.to_string()
}

// 17:52 In Karlsruhe, Gl 8 -> Gl 2
// Karlsruhe: Wg 15 Pl 88
