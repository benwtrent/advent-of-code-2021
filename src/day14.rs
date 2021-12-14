use std::collections::{BTreeMap, HashMap};

fn to_vec(input: &str) -> (&str, HashMap<String, String>) {
    let mut splt = input.split("\n\n");
    let polymer = splt.next().unwrap().trim();
    let rules: HashMap<_, _> = splt
        .next()
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let p: Vec<&str> = s
                .split("->")
                .filter(|ss| !ss.is_empty())
                .map(|ss| ss.trim())
                .collect();
            (p[0].to_string(), p[1].to_string())
        })
        .into_iter()
        .collect();
    (polymer, rules)
}

#[aoc(day14, part1)]
fn day14_1(input: &str) -> usize {
    let (polymer, rules) = to_vec(input);
    let polymer_split = polymer
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut growing_polymer: BTreeMap<Vec<String>, usize> = polymer_split
        .windows(2)
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, c| {
            *acc.entry(c.to_vec()).or_insert(0) += 1;
            acc
        });
    growing_polymer.insert(
        [String::from("0"), polymer_split.first().unwrap().clone()].to_vec(),
        1,
    );
    growing_polymer.insert(
        [polymer_split.last().unwrap().clone(), String::from("0")].to_vec(),
        1,
    );
    for _ in 0..10 {
        let mut new_polymer = BTreeMap::new();
        for (pair, count) in growing_polymer {
            if let Some(insert) = rules.get(&pair.concat()) {
                *new_polymer
                    .entry([pair[0].clone(), insert.clone()].to_vec())
                    .or_insert(0) += count;
                *new_polymer
                    .entry([insert.clone(), pair[1].clone()].to_vec())
                    .or_insert(0) += count;
            } else {
                new_polymer.insert(pair.clone(), count);
            }
        }
        growing_polymer = new_polymer;
    }
    let mut counts = HashMap::new();
    for (pair, count) in &growing_polymer {
        *counts.entry(pair[0].clone()).or_insert(0) += count
    }
    let mut max_v = 0;
    let mut min_v = usize::max_value();
    for (c, v) in counts {
        if c != "0" {
            min_v = min_v.min(v);
            max_v = max_v.max(v);
        }
    }
    max_v - min_v
}

#[aoc(day14, part2)]
fn day14_2(input: &str) -> usize {
    let (polymer, rules) = to_vec(input);
    let polymer_split = polymer
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut growing_polymer: BTreeMap<Vec<String>, usize> = polymer_split
        .windows(2)
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, c| {
            *acc.entry(c.to_vec()).or_insert(0) += 1;
            acc
        });
    growing_polymer.insert(
        [String::from("0"), polymer_split.first().unwrap().clone()].to_vec(),
        1,
    );
    growing_polymer.insert(
        [polymer_split.last().unwrap().clone(), String::from("0")].to_vec(),
        1,
    );
    for _ in 0..40 {
        let mut new_polymer = BTreeMap::new();
        for (pair, count) in growing_polymer {
            if let Some(insert) = rules.get(&pair.concat()) {
                *new_polymer
                    .entry([pair[0].clone(), insert.clone()].to_vec())
                    .or_insert(0) += count;
                *new_polymer
                    .entry([insert.clone(), pair[1].clone()].to_vec())
                    .or_insert(0) += count;
            } else {
                new_polymer.insert(pair.clone(), count);
            }
        }
        growing_polymer = new_polymer;
    }
    let mut counts = HashMap::new();
    for (pair, count) in &growing_polymer {
        *counts.entry(pair[0].clone()).or_insert(0) += count
    }
    let mut max_v = 0;
    let mut min_v = usize::max_value();
    for (c, v) in counts {
        if c != "0" {
            min_v = min_v.min(v);
            max_v = max_v.max(v);
        }
    }
    max_v - min_v
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_input() {
        assert_eq!(day14_1(&TEST_INPUT), 1588);
        assert_eq!(day14_2(&TEST_INPUT), 2188189693529);
    }
}
