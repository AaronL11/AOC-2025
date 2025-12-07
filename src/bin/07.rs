use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut queue = VecDeque::new();
    let mut grid = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            let mut row = vec![];
            for (c, chr) in line.chars().enumerate() {
                row.push(chr);
                if chr == 'S' {
                    queue.push_back((r + 1, c))
                }
            }
            row
        })
        .collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    let mut cnt = 0;
    while let Some((r, c)) = queue.pop_front() {
        let mut rp = r;
        while rp < n {
            if grid[rp][c] == '-' {
                break;
            }
            if grid[rp][c] == '^' {
                grid[rp][c] = '-';
                cnt += 1;
                if c >= 1 {
                    queue.push_back((rp, c - 1));
                }
                if c + 1 < m {
                    queue.push_back((rp, c + 1));
                }
                break;
            }
            rp += 1;
        }
    }
    Some(cnt)
}

fn search(
    (r, c): (usize, usize),
    grid: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(pcnt) = cache.get(&(r, c)) {
        *pcnt
    } else {
        let n = grid.len();
        let m = grid[0].len();
        let mut rp = r;
        let mut cnt = 0;
        while rp < n {
            if grid[rp][c] == '^' {
                // grid[rp][c] = '-';
                if c >= 1 {
                    cnt += search((rp, c - 1), grid, cache);
                }
                if c + 1 < m {
                    cnt += search((rp, c + 1), grid, cache);
                }
                break;
            }
            rp += 1;
        }
        if rp == n {
            cnt += 1;
        }
        cache.insert((r, c), cnt);
        cnt
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            let mut row = vec![];
            for (c, chr) in line.chars().enumerate() {
                row.push(chr);
                if chr == 'S' {
                    start = (r + 1, c);
                }
            }
            row
        })
        .collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    let mut cnt = 0;
    let mut cache = HashMap::new();
    Some(search(start, &grid, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
