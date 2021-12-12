use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

macro_rules! tree_set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = BTreeSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

struct Node<'a> {
    v: &'a str,
    edges: BTreeSet<&'a str>,
}

impl PartialEq<Self> for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(other.v)
    }
}

impl Eq for Node<'_> {}

impl PartialOrd<Self> for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.v.partial_cmp(other.v)
    }
}

impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(other.v)
    }
}

impl<'a> Node<'a> {
    fn new(v: &str) -> Node {
        Node {
            v,
            edges: BTreeSet::new(),
        }
    }

    fn add_node(&mut self, n: &'a str) -> bool {
        self.edges.insert(n)
    }

    fn paths_to_end(
        &self,
        already_visited: &HashMap<&str, usize>,
        nodes: &HashMap<&str, Node>,
    ) -> usize {
        if self.v == "end" {
            return 1;
        }
        let mut visited = already_visited.clone();
        *visited.entry(self.v).or_insert(0) += 1;
        self.edges
            .iter()
            .filter(|&&e| e.to_lowercase() != e || visited.get(e).unwrap_or(&0) < &1)
            .map(|&e| nodes.get(e).unwrap().paths_to_end(&visited, nodes))
            .sum()
    }

    fn paths_to_end_2(
        &self,
        already_visited: &HashMap<&str, usize>,
        nodes: &HashMap<&str, Node>,
    ) -> usize {
        if self.v == "end" {
            return 1;
        }
        let mut visited = already_visited.clone();
        *visited.entry(self.v).or_insert(0) += 1;
        let mut max_small_visitation = 0;
        for (&v, e) in &visited {
            if v != "start" && v.to_lowercase() == v {
                max_small_visitation = max_small_visitation.max(*e)
            }
        }
        self.edges
            .iter()
            .filter(|&&e| e != "start")
            .filter(|&&e| {
                e.to_lowercase() != e
                    || e == "end"
                    || already_visited.get(e).unwrap_or(&0) < &1
                    || (already_visited.get(e).unwrap_or(&0) < &2 && max_small_visitation < 2)
            })
            .map(|&e| nodes.get(e).unwrap().paths_to_end_2(&visited, nodes))
            .sum()
    }
}

fn to_vec(input: &str) -> HashMap<&str, Node> {
    let mut graph = HashMap::new();
    input.lines().for_each(|l| {
        let mut split = l.split("-").filter(|s| !s.is_empty());
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        graph
            .entry(first)
            .or_insert(tree_set![second])
            .insert(second);
        graph
            .entry(second)
            .or_insert(tree_set![first])
            .insert(first);
    });
    let mut nodes = HashMap::new();
    for (v, e) in graph {
        let mut n = Node::new(v);
        for edge in e {
            n.add_node(edge);
        }
        nodes.insert(v, n);
    }
    nodes
}

#[aoc(day12, part1)]
fn day12_1(input: &str) -> usize {
    let graph = to_vec(input);
    let root = graph.get("start").unwrap();
    let mut visited = HashMap::new();
    visited.insert(root.v, 1);
    root.paths_to_end(&visited, &graph)
}

#[aoc(day12, part2)]
fn day12_2(input: &str) -> usize {
    let graph = to_vec(input);
    let root = graph.get("start").unwrap();
    let mut visited = HashMap::new();
    visited.insert(root.v, 1);
    root.paths_to_end_2(&visited, &graph)
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const MOAR_TEST_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_input() {
        assert_eq!(day12_1(&TEST_INPUT), 10);
        assert_eq!(day12_1(&MOAR_TEST_INPUT), 226);
        assert_eq!(day12_2(&TEST_INPUT), 36);
        assert_eq!(day12_2(&MOAR_TEST_INPUT), 3509);
    }
}
