use std::collections::HashMap;

advent_of_code::solution!(11);

type Data<'a> = (&'a str, Vec<&'a str>);

fn parse<'a>(s: &'a str) -> Option<Data<'a>> {
    let (k, vs) = s.split_once(':')?;
    let vs = vs.trim().split_ascii_whitespace().collect::<Vec<_>>();
    Some((k, vs))
}

fn dfs1<'a>(
    v: &'a str,
    visited: &mut HashMap<&'a str, u64>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) -> u64 {
    if v == "out" {
        1
    } else if let Some(n) = visited.get(&v) {
        *n
    } else {
        let mut cnt = 0;
        if let Some(neighbours) = graph.get(&v) {
            for u in neighbours {
                cnt += dfs1(u, visited, graph);
            }
        }
        visited.insert(v, cnt);
        cnt
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = input.lines().flat_map(parse).collect::<HashMap<_, _>>();
    let mut visited = HashMap::new();
    Some(dfs1("you", &mut visited, &graph))
}

fn dfs2<'a>(
    v: &'a str,
    pth: u8,
    visited: &mut HashMap<(&'a str, u8), u64>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) -> u64 {
    if v == "out" && pth >= 2 {
        1
    } else if let Some(n) = visited.get(&(v, pth)) {
        *n
    } else {
        let mut npth = pth;
        if v == "dac" || v == "fft" {
            npth += 1;
        }
        let mut cnt = 0;
        if let Some(neighbours) = graph.get(&v) {
            for u in neighbours {
                cnt += dfs2(u, npth, visited, graph);
            }
        }
        visited.insert((v, pth), cnt);
        cnt
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = input.lines().flat_map(parse).collect::<HashMap<_, _>>();
    let mut visited = HashMap::new();
    Some(dfs2("svr", 0, &mut visited, &graph))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
