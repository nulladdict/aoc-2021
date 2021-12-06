fn main() {
    let fishes = include_str!("in")
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    dbg!(part1(&fishes));
    dbg!(part2(&fishes));
}

fn part1(fishes: &[usize]) -> u128 {
    populate(fishes, 80)
}

fn part2(fishes: &[usize]) -> u128 {
    populate(fishes, 256)
}

fn populate(fishes: &[usize], days: usize) -> u128 {
    let mut population = [0; 9];
    for &fish in fishes {
        population[fish] += 1;
    }
    for _ in 0..days {
        population.rotate_left(1);
        population[6] += population[8];
    }
    population.iter().sum()
}
