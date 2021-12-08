use itertools::Itertools;

fn main() {
    let input = include_str!("in")
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|x| x.split(" ").collect::<Vec<_>>())
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    dbg!(part1(&input));
    dbg!(part2(&input));
}

fn part1(input: &[(Vec<&str>, Vec<&str>)]) -> usize {
    input
        .iter()
        .flat_map(|(_, xs)| xs)
        .filter(|s| match s.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn part2(input: &[(Vec<&str>, Vec<&str>)]) -> usize {
    let mut sum = 0;

    for (patterns, outputs) in input.iter().cloned() {
        let abcdefg = find_permutation(&patterns);
        let c = abcdefg[2];
        let d = abcdefg[3];
        let e = abcdefg[4];

        let mut digits = String::with_capacity(4);
        for output in outputs.iter() {
            match output.len() {
                2 => digits.push('1'),
                3 => digits.push('7'),
                4 => digits.push('4'),
                7 => digits.push('8'),
                5 => {
                    if output.contains(e) {
                        digits.push('2');
                    } else if output.contains(c) {
                        digits.push('3');
                    } else {
                        digits.push('5');
                    }
                }
                6 => {
                    if output.contains(d) {
                        if output.contains(e) {
                            digits.push('6');
                        } else {
                            digits.push('9');
                        }
                    } else {
                        digits.push('0');
                    }
                }
                _ => unreachable!(),
            }
        }
        sum += digits.parse::<usize>().unwrap();
    }

    sum
}

fn find_permutation(patterns: &[&str]) -> Vec<char> {
    "abcdefg"
        .chars()
        .permutations(7)
        .find(|permutation| check_on(patterns, permutation))
        .unwrap()
}

macro_rules! contains {
    ($p:ident, $x:ident) => {
        $p.contains($x)
    };
    ($p:ident, $x:ident, $($y:ident),+) => {
        $p.contains($x) && contains!($p, $($y),+)
    }
}

fn check_on(patterns: &[&str], abcdefg: &[char]) -> bool {
    let a = abcdefg[0];
    let b = abcdefg[1];
    let c = abcdefg[2];
    let d = abcdefg[3];
    let e = abcdefg[4];
    let f = abcdefg[5];
    let g = abcdefg[6];

    for &pattern in patterns {
        match pattern.len() {
            2 => {
                if !contains!(pattern, c, f) {
                    return false;
                }
            }
            3 => {
                if !contains!(pattern, a, c, f) {
                    return false;
                }
            }
            4 => {
                if !contains!(pattern, b, c, d, f) {
                    return false;
                }
            }
            5 => {
                if !contains!(pattern, a, c, d, e, g)
                    && !contains!(pattern, a, c, d, f, g)
                    && !contains!(pattern, a, b, d, f, g)
                {
                    return false;
                }
            }
            6 => {
                if !contains!(pattern, a, b, c, e, f, g)
                    && !contains!(pattern, a, b, d, e, f, g)
                    && !contains!(pattern, a, b, c, d, f, g)
                {
                    return false;
                }
            }
            7 => {}
            _ => {
                unreachable!();
            }
        }
    }

    true
}
