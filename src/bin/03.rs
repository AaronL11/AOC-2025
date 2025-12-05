use std::cmp;

advent_of_code::solution!(3);

const MAXN: usize = 100;

fn parse(s: &str) -> Option<Vec<u8>> {
    Some(s.bytes().map(|b| b - 48).collect::<Vec<_>>())
}

fn solve(state: &mut u64, batteries: Vec<u8>) -> Option<u64> {
    let poss = (1..=9)
        .flat_map(|n| {
            let mut idx = batteries.len();
            let mut cnt = 0;
            for (i, &b) in batteries.iter().enumerate() {
                if b == n {
                    idx = cmp::min(idx, i);
                    cnt += 1;
                }
            }
            if cnt == 0 {
                None
            } else {
                Some((n as u64, idx, cnt))
            }
        })
        .last()?;
    let mut res = 0;
    let (b, idx, cnt) = poss;
    if cnt > 1 {
        res = b * 10 + b;
    } else {
        if let Some(max) = batteries[idx + 1..].iter().max() {
            res = b * 10 + *max as u64;
        } else if let Some(max) = batteries[..idx].iter().max() {
            res = *max as u64 * 10 + b;
        }
    }
    *state += res;
    Some(*state)
}

pub fn part_one(input: &str) -> Option<u64> {
    input.lines().flat_map(parse).scan(0, solve).last()
}

// fn print_matrix(&m: &[[(u64, usize); MAXN + 1]; MAXN + 1]) {
//     for row in m {
//         for (x, _) in row {
//             print!("{x}");
//         }
//         print!("\n");
//     }
// }

fn solver(n: u32, i: usize, j: usize, m: &[[(u64, usize); MAXN + 1]; MAXN + 1]) -> (u32, u64) {
    if n == 0 || i > j {
        (0, 0)
    } else if i == j {
        (1, m[i][j].0)
    } else {
        let (max, idx) = m[i][j];
        let idx = idx + 1;
        let (mut exp, resr) = solver(n - 1, idx + 1, j, m);
        let mut res = max * 10u64.pow(exp) + resr;
        exp += 1;
        if exp != n {
            let (exp2, resl) = solver(n - exp, i, idx - 1, m);
            res += resl * 10u64.pow(exp);
            exp += exp2;
        }
        (exp, res)
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut m = [[(0u64, 0usize); MAXN + 1]; MAXN + 1];
    input
        .lines()
        .flat_map(parse)
        .scan(0, |state, batteries| {
            m = [[(0u64, 0usize); MAXN + 1]; MAXN + 1];
            let l = batteries.len();
            m[1][1] = (batteries[0] as u64, 0);
            for i in 1..=l {
                m[i][i] = (batteries[i - 1] as u64, i - 1);
            }
            for i in 1..=l {
                for j in i + 1..=l {
                    let (l, ldx) = m[i][j - 1];
                    let (r, rdx) = m[j][j];
                    if l > r {
                        m[i][j] = (l, ldx);
                    } else if l == r {
                        m[i][j] = (r, cmp::min(ldx, rdx));
                    } else {
                        m[i][j] = (r, rdx)
                    }
                }
            }
            let (_, res) = solver(12, 1, l, &m);
            *state += res;
            Some(*state)
        })
        .last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
