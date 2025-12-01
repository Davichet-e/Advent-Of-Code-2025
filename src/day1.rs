#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl Direction {
    pub fn apply(&self, number: i32, delta: i32) -> i32 {
        match self {
            Direction::L => number - delta,
            Direction::R => number + delta,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Invalid direction character"),
        }
    }
}

pub fn day1() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../inputs/day1");
    let numbers: i32 = input
        .lines()
        .fold((50, 0), |(old_pos, acc), line| {
            let (direction, n_rotations): (Direction, i32) = (
                line.chars().next().unwrap().into(),
                line[1..].parse().unwrap(),
            )
                .into();
            let new_pos = direction.apply(old_pos, n_rotations).rem_euclid(100);
            if new_pos == 0 {
                (new_pos, acc + 1)
            } else {
                (new_pos, acc)
            }
        })
        .1;

    println!("{numbers}")
}

fn part2() {
    let input = include_str!("../inputs/day1");
    let numbers: i32 = input
        .lines()
        .fold((50, 0), |(old_pos, acc), line| {
            let (direction, n_rotations): (Direction, i32) = (
                line.chars().next().unwrap().into(),
                line[1..].parse().unwrap(),
            )
                .into();

            let new_pos = direction.apply(old_pos, n_rotations).rem_euclid(100);
            let n_total_rotations = n_rotations / 100;
            let did_it_click_0 = {
                let n = direction.apply(old_pos, n_rotations % (100));
                (old_pos != 0) && (n > 99 || n <= 0)
            };

            (new_pos, acc + n_total_rotations + did_it_click_0 as i32)
        })
        .1;

    println!("{numbers}")
}
