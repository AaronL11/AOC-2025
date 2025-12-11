use std::cmp;

advent_of_code::solution!(10);

const INF: u64 = 1 << 16;

struct Data {
    conf: u16,
    switches: Vec<u16>,
    jolts: Vec<u16>,
}

fn parse(s: &str) -> Option<Data> {
    let mut split = s.split_whitespace();
    let conf = split.next()?;
    let conf = conf
        .trim_matches(|chr| chr == '[' || chr == ']')
        .char_indices()
        .filter_map(|(idx, chr)| if chr == '#' { Some(idx as u8) } else { None })
        .fold(0, |acc, idx| acc | (1 << idx));
    let mut split = split.rev();
    let jolts = split
        .next()?
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .flat_map(|chr| u16::from_str_radix(chr, 10).ok())
        .collect::<Vec<_>>();
    let split = split.rev();
    let switches = split
        .map(|sub| {
            sub.trim_matches(|chr| chr == '(' || chr == ')')
                .split(',')
                .flat_map(|chr| u16::from_str_radix(chr, 10).ok())
                .fold(0u16, |acc, idx| acc | (1 << idx))
        })
        .collect::<Vec<_>>();
    Some(Data {
        conf,
        switches,
        jolts,
    })
}

fn solve1(data: Data) -> u64 {
    let Data { conf, switches, .. } = data;
    solver1(0, conf as usize, 0, &switches)
}

fn solver1(arr: usize, conf: usize, idx: usize, switches: &Vec<u16>) -> u64 {
    if arr == conf {
        0
    } else if idx >= switches.len() {
        INF
    // } else if cache[arr][idx] != -1 {
    //     cache[arr][idx] as u64
    } else {
        let switch = switches[idx] as usize;
        let take = solver1(arr ^ switch, conf, idx + 1, switches) + 1;
        let skip = solver1(arr, conf, idx + 1, switches);
        let res = cmp::min(take, skip);
        // cache[arr][idx] = res as i64;
        res
    }
}

fn solve2(data: Data) -> u64 {
    let Data {
        switches, jolts, ..
    } = data;
    solver2(0, conf as usize, 0, &switches)
}

fn solver2(arr: usize, conf: usize, idx: usize, switches: &Vec<u16>) -> u64 {
    if arr == conf {
        0
    } else if idx >= switches.len() {
        INF
    // } else if cache[arr][idx] != -1 {
    //     cache[arr][idx] as u64
    } else {
        let switch = switches[idx] as usize;
        let take = solver2(arr ^ switch, conf, idx + 1, switches) + 1;
        let skip = solver2(arr, conf, idx + 1, switches);
        let res = cmp::min(take, skip);
        // cache[arr][idx] = res as i64;
        res
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().flat_map(parse).map(solve1).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().flat_map(parse).map(solve2).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
