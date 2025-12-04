use std::collections::{BTreeMap, BTreeSet, VecDeque};

advent_of_code::solution!(4);

// const MAXN: usize = 10;
const MAXN: usize = 137;

type Grid = [[char; MAXN + 2]; MAXN + 2];

fn print_matrix(g: Grid) {
    for row in g {
        for c in row {
            print!("{c}");
        }
        print!("\n");
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = [['.'; MAXN + 2]; MAXN + 2];
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(r, line)| line.chars().enumerate().map(move |(c, ch)| (r, c, ch)))
        .for_each(|(r, c, ch)| grid[r + 1][c + 1] = ch);
    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    Some(
        (1..=MAXN)
            .flat_map(|r| (1..=MAXN).map(move |c| (r, c)))
            .filter(|(r, c)| grid[*r][*c] == '@')
            .map(|(r, c)| {
                dirs.iter()
                    .map(|(dx, dy)| (r as isize + dx, c as isize + dy))
                    .filter(|(rp, cp)| grid[*rp as usize][*cp as usize] == '@')
                    .count()
            })
            .filter(|cnt| *cnt < 4)
            .count() as u64,
    )
}

fn print_matrix2(g: [[i8; MAXN + 2]; MAXN + 2]) {
    for row in g {
        for c in row {
            if c >= 0 { print!("{c}") } else { print!(".") }
        }
        print!("\n");
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = [['.'; MAXN + 2]; MAXN + 2];
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(r, line)| line.chars().enumerate().map(move |(c, ch)| (r, c, ch)))
        .for_each(|(r, c, ch)| grid[r + 1][c + 1] = ch);
    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ngrid = [[-1i8; MAXN + 2]; MAXN + 2];
    let mut queue = VecDeque::new();
    let mut coords: BTreeMap<usize, BTreeSet<(usize, usize)>> = BTreeMap::new();
    for r in 1..=MAXN {
        for c in 1..=MAXN {
            if grid[r][c] == '@' {
                let mut cnt = 0;
                for (dx, dy) in dirs {
                    let (rp, cp) = (r as isize + dx, c as isize + dy);
                    if grid[rp as usize][cp as usize] == '@' {
                        cnt += 1;
                    }
                }
                if let Some(set) = coords.get_mut(&cnt) {
                    set.insert((r, c));
                } else {
                    let mut set = BTreeSet::new();
                    set.insert((r, c));
                    coords.insert(cnt, set);
                }
                ngrid[r][c] = cnt as i8;
                if cnt < 4 {
                    queue.push_back((r, c));
                }
            }
        }
    }
    let mut cnt = 0;
    while let Some((r, c)) = queue.pop_front() {
        if ngrid[r][c] < 0 || ngrid[r][c] >= 4 {
            continue;
        }
        ngrid[r][c] = -1;
        cnt += 1;
        for (dx, dy) in dirs {
            let (rp, cp) = (r as isize + dx, c as isize + dy);
            let (rp, cp) = (rp as usize, cp as usize);
            if ngrid[rp][cp] >= 0 {
                ngrid[rp][cp] -= 1;
                queue.push_back((rp, cp));
            }
        }
    }
    Some(cnt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
