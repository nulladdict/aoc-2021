use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward,
    Down,
    Up,
}
use Command::*;

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Forward),
            "down" => Ok(Down),
            "up" => Ok(Up),
            _ => Err(()),
        }
    }
}

type Move = (Command, i64);

fn main() {
    let moves = include_str!("in")
        .lines()
        .map(|line| {
            let (command, amount) = line.split(" ").collect_tuple().unwrap();
            let amount = amount.parse().unwrap();
            let command = command.parse().unwrap();
            (command, amount)
        })
        .collect::<Vec<_>>();

    dbg!(part1(&moves));
    dbg!(part2(&moves));
}

fn part1(moves: &[Move]) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    for (c, x) in moves {
        match c {
            Forward => horizontal += x,
            Down => depth += x,
            Up => depth -= x,
        }
    }
    horizontal * depth
}

fn part2(moves: &[Move]) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (c, x) in moves {
        match c {
            Forward => {
                horizontal += x;
                depth += aim * x
            }
            Down => aim += x,
            Up => aim -= x,
        }
    }
    horizontal * depth
}
