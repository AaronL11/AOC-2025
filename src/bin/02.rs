use std::collections::HashSet;

advent_of_code::solution!(2);

fn parse(s: &str) -> Option<(u64, u64)> {
    let ids = s
        .split('-')
        .flat_map(|c| u64::from_str_radix(c, 10).ok())
        .collect::<Vec<_>>();
    Some((ids[0], ids[1]))
}

fn solve1(state: &mut u64, (l, r): (u64, u64)) -> Option<u64> {
    let (mut dl, mut dr) = (l.ilog10() + 1, r.ilog10() + 1);
    if dl % 2 == 1 {
        dl += 1;
    }
    if dr % 2 == 1 {
        dr -= 1;
    }
    if dl <= dr {
        for exp in (dl..=dr).step_by(2) {
            let b = 10u64.pow(exp / 2) + 1;
            let (n, m) = (l / b, r / b);
            for k in n..=m {
                let prod = k * b;
                if l <= prod && prod <= r && (prod.ilog10() + 1) % 2 == 0 {
                    *state += prod;
                }
            }
        }
    }
    Some(*state)
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .trim()
        .split(',')
        .flat_map(parse)
        .scan(0, solve1)
        .last()
}

pub fn part_two(input: &str) -> Option<u64> {
    let multiples = (1u32..=19)
        .map(|n| {
            let mut factors = vec![];
            let sqrt = (n as f64).sqrt().ceil() as u32;
            for m in 1..sqrt {
                if n % m == 0 {
                    factors.push(m);
                    factors.push(n / m);
                }
            }
            if sqrt * sqrt == n {
                factors.push(sqrt);
            }
            factors.sort();
            factors
        })
        .collect::<Vec<_>>();

    let mut seen: HashSet<u64> = HashSet::new();
    input.trim().split(',').flat_map(parse).for_each(|(l, r)| {
        let (dl, dr) = (l.ilog10() + 1, r.ilog10() + 1);
        // dbg!(dl, dr);
        if dl <= dr {
            for exp in dl..=dr {
                for d in multiples[exp as usize - 1].iter().skip(1) {
                    let b = (0..*d).map(|j| 10u64.pow(j * exp / d)).sum::<u64>();
                    // dbg!(exp, d, b);
                    let (n, m) = (l / b, r / b);
                    for k in n..=m {
                        if *d == exp && k % 10 == 0 {
                            continue;
                        }
                        let prod = k * b;
                        if l <= prod && prod <= r {
                            seen.insert(prod);
                        }
                    }
                }
            }
        }
    });
    Some(seen.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
