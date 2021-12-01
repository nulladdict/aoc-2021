use itertools::Itertools;
use std::io::BufRead;

fn main() {
    let measurements = std::io::stdin()
        .lock()
        .lines()
        .map(|m| {
            let m = m.unwrap();
            let m = m.trim();
            m.parse::<u16>().unwrap()
        })
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
    xs.iter()
        .tuple_windows()
        .filter(|(x, y)| y > x)
        .count()
}
