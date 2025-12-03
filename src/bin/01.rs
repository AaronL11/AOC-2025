advent_of_code::solution!(1);

const M: i64 = 100;

fn parse1(s: &str) -> Option<i64> {
    let sgn = if s.starts_with("R") { 1 } else { -1 };
    let n = i64::from_str_radix(s.get(1..).unwrap(), 10).ok()?;
    let res = sgn * n;
    // dbg!(sgn, n, res);
    Some((M + res) % M)
}

fn parse2(s: &str) -> Option<i64> {
    let sgn = if s.starts_with("R") { 1 } else { -1 };
    let n = i64::from_str_radix(s.get(1..).unwrap(), 10).ok()?;
    let res = sgn * n;
    // dbg!(sgn, n, res);
    Some(res)
}

#[derive(Clone, Copy, Debug)]
struct State {
    pw: u64,
    acc: i64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .flat_map(parse1)
        .scan(State { pw: 0, acc: 50 }, |state, x| {
            // dbg!(&state, x);
            state.acc += x;
            state.acc %= M;
            if state.acc == 0 {
                state.pw += 1;
            }
            Some(*state)
        })
        .last();
    Some(res?.pw)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .flat_map(parse2)
        .scan(State { pw: 0, acc: 50 }, |state, x| {
            let mut y = x;
            state.pw += (y / M).abs() as u64;
            y %= M;
            let z = (M + state.acc + y) % M;
            // dbg!(&state, x, y, z);
            if state.acc != 0 {
                if z == 0 {
                    state.pw += 1;
                } else if y < 0 && z > state.acc {
                    state.pw += 1;
                } else if y > 0 && z < state.acc {
                    state.pw += 1;
                }
            }
            state.acc = z;
            Some(*state)
        })
        .last();
    Some(res?.pw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
