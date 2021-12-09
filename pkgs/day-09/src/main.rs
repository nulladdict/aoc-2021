use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i8, i8);
type Grid = HashMap<Point, u32>;

fn main() {
    let grid = include_str!("in")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, h)| {
                let height = h.to_digit(10).unwrap();
                ((x as i8, y as i8), height)
            })
        })
        .collect::<Grid>();

    dbg!(part1(&grid));
    dbg!(part2(&grid));
}

fn part1(grid: &Grid) -> u32 {
    get_lows(grid).map(|(_, height)| height + 1).sum()
}

fn get_lows(grid: &Grid) -> impl Iterator<Item = (&Point, &u32)> {
    grid.iter().filter(|(p, h)| is_low(grid, **p, **h))
}

fn is_low(grid: &Grid, point: Point, height: u32) -> bool {
    let (x, y) = point;
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .iter()
        .filter_map(|p| grid.get(p))
        .all(|&p| p > height)
}

fn part2(grid: &Grid) -> usize {
    let mut basins = get_lows(grid)
        .map(|(p, _)| basin_size(grid, *p))
        .collect::<Vec<_>>();
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn basin_size(grid: &Grid, point: Point) -> usize {
    let (x, y) = point;
    let mut basin = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    while let Some((x, y)) = queue.pop_front() {
        if let Some(0..=8) = grid.get(&(x, y)) {
            if basin.insert((x, y)) {
                queue.push_back((x + 1, y));
                queue.push_back((x - 1, y));
                queue.push_back((x, y + 1));
                queue.push_back((x, y - 1));
            }
        }
    }

    basin.len()
}
