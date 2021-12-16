use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Copy, Clone)]
struct Node {
    val: usize,
    pos: (usize, usize),
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Cost {
    cost: usize,
    pos: (usize, usize),
}

impl PartialOrd<Self> for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

fn neighbors(ij: &(usize, usize), max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    let i = ij.0;
    let j = ij.1;
    let mut n = vec![];
    if i >= 1 {
        n.push((i - 1, j));
    }
    if i < max_i {
        n.push((i + 1, j));
    }
    if j >= 1 {
        n.push((i, j - 1));
    }
    if j < max_j {
        n.push((i, j + 1));
    }
    n
}

fn to_vec(input: &str) -> Vec<Vec<Node>> {
    let mut v = vec![];
    for (i, l) in input.lines().enumerate() {
        let mut o = vec![];
        for (j, v) in l.split("").filter(|s| !s.is_empty()).enumerate() {
            o.push(Node {
                val: v.parse().unwrap(),
                pos: (i, j),
            });
        }
        v.push(o);
    }
    v
}

#[aoc(day15, part1)]
fn day15_1(input: &str) -> usize {
    let nodes = to_vec(input);
    let max_i = nodes.len() - 1;
    let max_j = nodes.first().unwrap().len() - 1;
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..=max_i {
        for j in 0..=max_j {
            costs.insert((i, j), usize::max_value());
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(Cost {
        pos: (0, 0),
        cost: 0,
    });
    costs.insert((0, 0), 0);

    while let Some(Cost { pos, cost }) = heap.pop() {
        if pos == (max_i, max_j) {
            return cost;
        }
        if cost > *costs.get(&pos).unwrap() {
            continue;
        }

        for n in neighbors(&pos, max_i, max_j) {
            let node_cost = nodes[n.0][n.1];
            let next_cost = cost + node_cost.val;
            if next_cost < *costs.get(&n).unwrap() {
                heap.push(Cost {
                    pos: n,
                    cost: next_cost,
                });
                costs.insert(n.clone(), next_cost);
            }
        }
    }
    0
}

#[aoc(day15, part2)]
fn day15_2(input: &str) -> usize {
    let original_nodes = to_vec(input);
    let max_i = 5 * (original_nodes.len()) - 1;
    let max_j = 5 * (original_nodes.first().unwrap().len()) - 1;
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..=max_i {
        for j in 0..=max_j {
            costs.insert((i, j), usize::max_value());
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(Cost {
        pos: (0, 0),
        cost: 0,
    });
    costs.insert((0, 0), 0);

    while let Some(Cost { pos, cost }) = heap.pop() {
        if pos == (max_i, max_j) {
            return cost;
        }
        if cost > *costs.get(&pos).unwrap() {
            continue;
        }

        for n in neighbors(&pos, max_i, max_j) {
            let (y, y_plus) = if n.0 > (original_nodes.len() - 1) {
                let y = n.0 % (original_nodes.len());
                let y_plus = n.0 / (original_nodes.len());
                (y, y_plus)
            } else {
                (n.0, 0)
            };
            let (x, x_plus) = if n.1 > (original_nodes.first().unwrap().len() - 1) {
                let x = n.1 % (original_nodes.first().unwrap().len());
                let x_plus = n.1 / (original_nodes.first().unwrap().len());
                (x, x_plus)
            } else {
                (n.1, 0)
            };
            let mut node_cost = original_nodes[y][x].val + x_plus + y_plus;
            while node_cost > 9 {
                node_cost -= 9
            }
            let next_cost = cost + node_cost;
            if next_cost < *costs.get(&n).unwrap() {
                heap.push(Cost {
                    pos: n,
                    cost: next_cost,
                });
                costs.insert(n.clone(), next_cost);
            }
        }
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_input() {
        assert_eq!(day15_1(&TEST_INPUT), 40);
        assert_eq!(day15_2(&TEST_INPUT), 315);
    }
}
