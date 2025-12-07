advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Op {
    Mul,
    Add,
}

#[derive(Clone, Copy)]
enum Data {
    Operation(Op),
    Num(u64),
    Partial(Op, u64),
}

fn parse(s: &str) -> Option<Vec<Data>> {
    Some(
        s.split_ascii_whitespace()
            .map(|b| match b {
                "+" => Data::Operation(Op::Add),
                "*" => Data::Operation(Op::Mul),
                _ => Data::Num(u64::from_str_radix(b, 10).unwrap()),
            })
            .collect::<Vec<_>>(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    let results = input
        .lines()
        .flat_map(parse)
        .rev()
        .reduce(|mut state, vdata| {
            for (d1, d2) in state.iter_mut().zip(vdata.iter()) {
                if let Data::Num(m) = d2 {
                    match d1 {
                        Data::Operation(op) => *d1 = Data::Partial(*op, *m),
                        Data::Partial(op, n) => {
                            if let Op::Add = op {
                                *n += *m;
                            } else {
                                *n *= *m;
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    unreachable!()
                }
            }
            state
        })?;
    Some(
        results
            .iter()
            .map(|data| {
                if let Data::Partial(_, n) = data {
                    *n
                } else {
                    unreachable!()
                }
            })
            .sum::<u64>(),
    )
}

type Grid = Vec<Vec<char>>;

fn find_blocks(grid: &Grid) -> Vec<usize> {
    let mut blocks = vec![0];
    let row = &grid[0];
    let rows = grid.len();
    let cols = row.len();
    for c in 0..cols {
        if (0..rows - 1).all(move |r| grid[r][c] == ' ') {
            blocks.push(c + 1);
        }
    }
    blocks.push(row.len() + 1);
    blocks
}

fn compute_blocks(blocks: &Vec<usize>, grid: &Grid) -> u64 {
    let mut answers = vec![];
    let n = grid.len();
    for (l, r) in blocks[..].windows(2).flat_map(|block| {
        if let &[l, r] = block {
            Some((l, r - 1))
        } else {
            None
        }
    }) {
        let mut nblock = vec![];
        for c in l..r {
            for i in 0..n - 1 {
                let chr = grid[i][c];
                if chr == ' ' {
                    if nblock.len() == c - l + 1 {
                        break;
                    } else {
                        continue;
                    }
                }
                let chr = (chr as u8 - 48) as u64;
                if let Some(n) = nblock.get_mut(c - l) {
                    *n = *n * 10 + chr;
                } else {
                    nblock.push(chr);
                }
            }
        }
        if grid[n - 1][l] == '+' {
            answers.push(nblock.iter().sum());
        } else {
            answers.push(nblock.iter().product());
        }
    }
    answers.iter().sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let blocks = find_blocks(&grid);
    Some(compute_blocks(&blocks, &grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
