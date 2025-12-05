advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy)]
enum Data {
    Range(u64, u64),
    ID(u64),
}

fn parse(s: &str) -> Option<Data> {
    if let Some((l, r)) = s.split_once('-') {
        Some(Data::Range(
            u64::from_str_radix(l, 10).ok()?,
            u64::from_str_radix(r, 10).ok()?,
        ))
    } else {
        Some(Data::ID(u64::from_str_radix(s, 10).ok()?))
    }
}

fn parse2(s: &str) -> Option<(u64, u64)> {
    let (l, r) = s.split_once('-')?;
    Some((
        u64::from_str_radix(l, 10).ok()?,
        u64::from_str_radix(r, 10).ok()?,
    ))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges = vec![];
    let mut ids = vec![];
    input
        .trim()
        .lines()
        .flat_map(parse)
        .for_each(|data| match data {
            Data::Range(l, r) => ranges.push((l, r)),
            Data::ID(id) => ids.push(id),
        });
    Some(
        ids.iter()
            .filter_map(|id| {
                if ranges.iter().any(|(l, r)| l <= id && id <= r) {
                    Some(id)
                } else {
                    None
                }
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges = input.trim().lines().flat_map(parse2).collect::<Vec<_>>();
    ranges.sort();
    let mut nranges = vec![ranges[0]];
    for &(l, r) in &ranges[1..] {
        let (pl, pr) = nranges.pop()?;
        if pr < l {
            nranges.push((pl, pr));
            nranges.push((l, r));
        } else if r < pr {
            nranges.push((pl, pr));
        } else {
            nranges.push((pl, r));
        }
    }
    let mut cnt = 0;
    for (l, r) in nranges {
        cnt += r - l + 1;
    }
    Some(cnt)
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
        assert_eq!(result, Some(14));
    }
}
