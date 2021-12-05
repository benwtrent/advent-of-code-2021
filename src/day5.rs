use std::cmp::max;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Line {
    start: Point,
    end: Point,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let points: Vec<Point> = s
            .split("->")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| {
                let xy: Vec<_> = x
                    .split(",")
                    .filter(|y| !y.is_empty())
                    .map(|y| y.parse().unwrap())
                    .collect();
                Point { x: xy[0], y: xy[1] }
            })
            .collect();
        Line {
            start: points[0],
            end: points[1],
        }
    }
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn point_set(&self) -> HashSet<Point> {
        let sx = min(self.start.x, self.end.x);
        let ex = max(self.start.x, self.end.x);
        let sy = min(self.start.y, self.end.y);
        let ey = max(self.start.y, self.end.y);
        if sx == ex {
            (sy..=ey).map(|y| Point { x: self.start.x, y }).collect()
        } else if sy == ey {
            (sx..=ex).map(|x| Point { x, y: self.start.y }).collect()
        } else {
            let xs: Vec<_> = if sx == self.start.x {
                (sx..=ex).collect()
            } else {
                (sx..=ex).rev().collect()
            };
            let ys: Vec<_> = if sy == self.start.y {
                (sy..=ey).collect()
            } else {
                (sy..=ey).rev().collect()
            };
            assert_eq!(ys.len(), xs.len());
            (0..xs.len())
                .map(|i| Point { x: xs[i], y: ys[i] })
                .collect()
        }
    }
}

#[aoc_generator(day5)]
fn to_vec(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.into())
        .collect()
}

#[aoc(day5, part1)]
fn day5_1(input: &Vec<Line>) -> usize {
    let mut point_count = HashMap::new();
    for l in input {
        if l.is_horizontal_or_vertical() {
            for pt in l.point_set() {
                let e = point_count.entry(pt).or_insert(0);
                *e += 1
            }
        }
    }
    point_count.values().filter(|&c| *c > 1).count()
}

#[aoc(day5, part2)]
fn day5_2(input: &Vec<Line>) -> usize {
    let mut point_count = HashMap::new();
    for l in input {
        for pt in l.point_set() {
            let e = point_count.entry(pt).or_insert(0);
            *e += 1
        }
    }
    point_count.values().filter(|&c| *c > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day5_1(&input), 5);
        assert_eq!(day5_2(&input), 12);
    }
}
