use std::{cmp::Ordering, collections::BinaryHeap};

advent_of_code::solution!(9);

const MAXN: usize = 8;
// const MAXN: usize = 497;

type Point = (i64, i64);

fn area((x1, y1): &Point, (x2, y2): &Point) -> u64 {
    (((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)) as u64
}

fn parse(s: &str) -> Option<Point> {
    let (x, y) = s.split_once(',')?;
    Some((
        i64::from_str_radix(x, 10).ok()? - 1,
        i64::from_str_radix(y, 10).ok()? - 1,
    ))
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = input.lines().flat_map(parse).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    let n = points.len();
    for i in 0..n {
        for j in i + 1..n {
            let (p1, p2) = (points[i], points[j]);
            if p1.0 != p2.0 && p1.1 != p2.1 {
                heap.push(area(&p1, &p2));
            }
        }
    }
    heap.pop()
}

type Output = [(Option<usize>, Option<usize>); MAXN];

fn get_lines<F, G>(points: &mut Vec<Point>, compare: F, eq: G) -> Output
where
    F: FnMut(&Point, &Point) -> Ordering,
    G: Fn(&Point, &Point) -> bool,
{
    let mut nxs = [(None, None); MAXN];
    points.sort_by(compare);
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if eq(&p1, &p2) {
            let (nd, _) = nxs[i];
            nxs[i] = (nd, Some(i + 1));
            let (_, nu) = nxs[i];
            nxs[i + 1] = (Some(i), nu)
        }
    }
    nxs
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points = input.lines().flat_map(parse).collect::<Vec<_>>();
    let mut nxs = [(None, None); MAXN];
    let mut nys = [(None, None); MAXN];
    let nxs = get_lines(&mut points, |a, b| a.cmp(&b), |p1, p2| p1.0 == p2.0);
    let nys = get_lines(
        &mut points,
        |(x1, y1), (x2, y2)| (y1, x1).cmp(&(y2, x2)),
        |p1, p2| p1.0 == p2.0,
    );
    points.sort();
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if p1.0 == p2.0 {
            let (nd, _) = nxs[i];
            nxs[i] = (nd, Some(i + 1));
            let (_, nu) = nxs[i];
            nxs[i + 1] = (Some(i), nu)
        }
    }
    points.sort_by(|(_, y1), (_, y2)| y1.cmp(y2));
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        for j in i + 1..n {
            let (p1, p2) = (points[i], points[j]);
            if p1.0 != p2.0 && p1.1 != p2.1 {
                heap.push(area(&p1, &p2));
            }
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
