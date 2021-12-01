use itertools::Itertools;

fn main() {
    let measurements = include_str!("in")
        .lines()
        .map(|m| m.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let part1 = count_increase(&measurements);
    dbg!(part1);

    let measurements = measurements
        .iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .collect::<Vec<_>>();

    let part2 = count_increase(&measurements);
    dbg!(part2);
}

fn count_increase<T: Ord>(xs: &[T]) -> usize {
    xs.iter().tuple_windows().filter(|(x, y)| y > x).count()
}
