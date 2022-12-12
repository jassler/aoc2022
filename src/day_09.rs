use std::cmp::max;
use std::collections::BTreeSet;
use std::ops;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Rope {
    knots: Vec<Point>,
    visited: BTreeSet<Point>
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
        }
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    // diagonal counts as 1!
    fn distance_to(self, other: &Point) -> isize {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    fn move_towards(&mut self, other: &Point) {
        if other.x < self.x {
            self.x -= 1;
        } else if other.x > self.x {
            self.x += 1;
        }

        if other.y < self.y {
            self.y -= 1;
        } else if other.y > self.y {
            self.y += 1;
        }
    }
}

impl ops::Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Point) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<isize> for Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Rope {
    fn new() -> Rope {
        Rope {
            knots: vec![Point::new(), Point::new()],
            visited: BTreeSet::try_from([Point::new()]).unwrap(),
        }
    }

    fn new_with_size(size: usize) -> Rope {
        Rope {
            knots: vec![Point::new(); size],
            visited: BTreeSet::try_from([Point::new()]).unwrap(),
        }
    }

    fn move_head(&mut self, amount: &Point) {
        let mut amount = Point { ..*amount };
        let dir = match &amount {
            p if p.x < 0 => Point { x:-1, y: 0 },
            p if p.x > 0 => Point { x: 1, y: 0 },
            p if p.y < 0 => Point { x: 0, y:-1 },
            p if p.y > 0 => Point { x: 0, y: 1 },
            _ => return,
        };

        while !amount.is_origin() {
            self.knots[0] += &dir;
            amount -= &dir;

            let mut prev = self.knots[0];

            for i in 1..self.knots.len() {
                if self.knots[i].distance_to(&prev) > 1 {
                    self.knots[i].move_towards(&prev);
                }
                prev = self.knots[i];
            }
            println!("{:?}", self.knots);
            self.visited.insert(self.knots[self.knots.len() - 1]);
        }
    }
}

fn parse_line(line: &str) -> Point {
    let mut split = line.split(' ');
    let p = match split.next().unwrap() {
        "U" => Point { x:  0, y: -1 },
        "D" => Point { x:  0, y:  1 },
        "L" => Point { x: -1, y:  0 },
        "R" => Point { x:  1, y:  0 },
        _ => panic!("Unknown direction"),
    };

    p * split.next().unwrap().parse::<isize>().unwrap()
}

pub fn part_01(input: &str) -> String {
    let mut rope = Rope::new();

    input
        .lines()
        .map(|l| parse_line(l))
        .for_each(|p| rope.move_head(&p));

    rope.visited.len().to_string()
}

pub fn part_02(input: &str) -> String {
    let mut rope = Rope::new_with_size(10);

    input
        .lines()
        .map(|l| parse_line(l))
        .for_each(|p| rope.move_head(&p));

    rope.visited.len().to_string()
}

// 17:52 In Karlsruhe, Gl 8 -> Gl 2
// Karlsruhe: Wg 15 Pl 88
