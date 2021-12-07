fn main() {
    let mut positions = include_str!("in")
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    positions.sort();

    dbg!(part1(&positions));
    dbg!(part2(&positions));
}

fn part1(positions: &[i64]) -> i64 {
    let min = positions[0];
    let max = positions[positions.len() - 1];

    (min..max)
        .map(|offset| {
            positions
                .iter()
                .map(|position| (position - offset).abs())
                .sum()
        })
        .min()
        .unwrap()
}

fn part2(positions: &[i64]) -> i64 {
    let min = positions[0];
    let max = positions[positions.len() - 1];

    (min..max)
        .map(|offset| {
            positions
                .iter()
                .map(|position| {
                    let diff = (position - offset).abs();
                    diff * (diff + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
