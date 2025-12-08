use std::{
    collections::{BTreeSet, BinaryHeap},
    mem, ops,
};
advent_of_code::solution!(8);

const MAXP: usize = 1000;
const TOPN: usize = 3;

type Point = [i64; 3];

#[derive(Debug, PartialEq)]
struct NonNan(f64);

impl Eq for NonNan {}

impl PartialOrd for NonNan {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for NonNan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.partial_cmp(&self.0).unwrap()
    }
}

fn d(u: &Point, v: &Point) -> NonNan {
    NonNan(
        (u.iter()
            .zip(v.iter())
            .map(|(ui, vi)| (ui - vi).pow(2))
            .sum::<i64>() as f64)
            .sqrt(),
    )
}

#[derive(Debug, Clone)]
struct DSU {
    array: Vec<usize>,
    cnt: u64,
    size: Vec<u64>,
    last_cont: (usize, usize),
}

impl ops::Index<usize> for DSU {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        self.array.index(index)
    }
}

impl ops::IndexMut<usize> for DSU {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.array.index_mut(index)
    }
}

impl DSU {
    fn from(cnt: usize) -> Self {
        let array = (0..cnt).collect::<Vec<_>>();
        let size = vec![1; cnt];
        let cnt = cnt as u64;
        DSU {
            array,
            cnt,
            size,
            last_cont: (0, 0),
        }
    }

    fn find(&self, v: &usize) -> usize {
        let u = self[*v];
        if u == *v { u } else { self.find(&u) }
    }

    fn join(&mut self, i: &usize, j: &usize) {
        let mut x = self.find(i);
        let mut y = self.find(j);
        if x != y {
            if self.size[x] < self.size[y] {
                mem::swap(&mut x, &mut y);
            }
            self[y] = x;
            if self.cnt == 2 {
                self.last_cont = (*i, *j);
            }
            self.cnt -= 1;
            self.size[x] += self.size[y];
        }
    }

    fn top(&self, n: usize) -> Vec<u64> {
        let mut sizes = BTreeSet::new();
        for size in &self.size {
            sizes.insert(*size);
        }
        sizes.into_iter().rev().take(n).collect::<Vec<_>>()
    }
}

fn parse(s: &str) -> Point {
    let mut point = [0; 3];
    s.split(',')
        .enumerate()
        .for_each(|(i, p)| point[i] = i64::from_str_radix(p, 10).unwrap());
    point
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut heap = BinaryHeap::new();
    let points = input.trim().lines().map(parse).collect::<Vec<_>>();
    let mut dsu = DSU::from(points.len());
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if i != j {
                let (u, v) = (points[i], points[j]);
                heap.push((d(&u, &v), i, j));
            }
        }
    }
    let mut n = 1;
    while let Some((_, i, j)) = heap.pop() {
        if n > MAXP {
            break;
        }
        dsu.join(&i, &j);
        n += 1;
    }
    let top = dsu.top(TOPN);
    Some(top.iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut heap = BinaryHeap::new();
    let points = input.trim().lines().map(parse).collect::<Vec<_>>();
    let mut dsu = DSU::from(points.len());
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if i != j {
                let (u, v) = (points[i], points[j]);
                heap.push((d(&u, &v), i, j));
            }
        }
    }
    while let Some((_, i, j)) = heap.pop() {
        if dsu.cnt == 1 {
            break;
        }
        dsu.join(&i, &j);
    }
    let (i, j) = dsu.last_cont;
    let (x, y) = (points[i][0] as u64, points[j][0] as u64);
    Some(x * y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
