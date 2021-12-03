const BIT_LENGTH: usize = 12;
const MASK: usize = (1 << BIT_LENGTH) - 1;

fn main() {
    let numbers = include_str!("in")
        .lines()
        .map(|s| {
            assert_eq!(s.len(), BIT_LENGTH);
            usize::from_str_radix(s, 2).unwrap()
        })
        .collect::<Vec<_>>();

    dbg!(part1(&numbers));
    dbg!(part2(&numbers));
}

fn part1(numbers: &[usize]) -> usize {
    let mut gamma_rate = 0;
    let size = numbers.len();
    for offset in 0..BIT_LENGTH {
        let ones = numbers
            .iter()
            .filter(|&&number| bit_at(number, offset) == 1)
            .count();
        if ones > size / 2 {
            gamma_rate += 1 << offset;
        }
    }
    let epsilon_rate = !gamma_rate & MASK;
    gamma_rate * epsilon_rate
}

fn bit_at(number: usize, offset: usize) -> u8 {
    (number >> offset & 1) as u8
}

fn part2(numbers: &[usize]) -> usize {
    let oxygen_rating = bit_fuckery(numbers, most_common);
    let c02_rating = bit_fuckery(numbers, least_common);
    oxygen_rating * c02_rating
}

fn most_common(ones: usize, zeros: usize) -> u8 {
    (ones >= zeros) as u8
}

fn least_common(ones: usize, zeros: usize) -> u8 {
    (ones < zeros) as u8
}

fn bit_fuckery<C>(numbers: &[usize], criteria: C) -> usize
where
    C: Fn(usize, usize) -> u8,
{
    let mut numbers = numbers.to_vec();

    for offset in (0..BIT_LENGTH).rev() {
        let size = numbers.len();
        let ones = numbers
            .iter()
            .filter(|&&number| bit_at(number, offset) == 1)
            .count();
        let zeros = size - ones;
        let the_bit = criteria(ones, zeros);

        numbers.retain(|&number| bit_at(number, offset) == the_bit);
        if numbers.len() == 1 {
            break;
        }
    }

    numbers[0]
}
