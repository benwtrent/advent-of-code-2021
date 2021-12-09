use std::collections::{HashSet, VecDeque};

#[aoc_generator(day9)]
fn to_vec(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split("")
                .filter(|s| !s.is_empty())
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

fn get_neighbors(p: &Point, input: &Vec<Vec<usize>>, ensure_less_than: usize) -> Vec<Point> {
    let i = p.y;
    let j = p.x;
    let mut neighbors = vec![];
    if i < input.len() - 1 && input[i + 1][j] < ensure_less_than {
        neighbors.push(Point { y: i + 1, x: j });
    }
    if j < input[i].len() - 1 && input[i][j + 1] < ensure_less_than {
        neighbors.push(Point { y: i, x: j + 1 });
    }
    if i > 0 && input[i - 1][j] < ensure_less_than {
        neighbors.push(Point { y: i - 1, x: j });
    }
    if j > 0 && input[i][j - 1] < ensure_less_than {
        neighbors.push(Point { y: i, x: j - 1 });
    }
    neighbors
}

#[aoc(day9, part1)]
fn day9_1(input: &Vec<Vec<usize>>) -> usize {
    let mut low_sums = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let v = input[i][j];
            let neighbors = get_neighbors(&Point { y: i, x: j }, input, 10);
            if neighbors.iter().all(|n| input[n.y][n.x] > v) {
                low_sums += v + 1;
            }
        }
    }
    low_sums
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[aoc(day9, part2)]
fn day9_2(input: &Vec<Vec<usize>>) -> usize {
    let mut basin_sizes = vec![];
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let v = input[i][j];
            let neighbors = get_neighbors(&Point { y: i, x: j }, input, 9);
            if neighbors.iter().all(|n| input[n.y][n.x] > v) {
                let mut explored = HashSet::new();
                let mut basin_size = neighbors.len() + 1;
                let mut explore = VecDeque::new();
                for n in neighbors {
                    explore.push_back(n);
                    explored.insert(n.clone());
                }
                while !explore.is_empty() {
                    if let Some(p) = explore.pop_back() {
                        let p_v = input[p.y][p.x];
                        let p_neighbors = get_neighbors(&p, input, 9);
                        for p_n in p_neighbors {
                            let n_v = input[p_n.y][p_n.x];
                            if n_v > p_v && n_v < 9 && !explored.contains(&p_n) {
                                explore.push_back(p_n);
                                explored.insert(p_n.clone());
                                basin_size += 1
                            }
                        }
                    }
                }
                basin_sizes.push(basin_size);
            }
        }
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day9_1(&input), 15);
        assert_eq!(day9_2(&input), 1134);
    }
}
