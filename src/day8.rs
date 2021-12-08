use std::collections::{BTreeSet, HashMap};

#[aoc_generator(day8)]
fn to_vec(input: &str) -> Vec<(Vec<BTreeSet<char>>, Vec<BTreeSet<char>>)> {
    input
        .lines()
        .map(|s| {
            let pair: Vec<&str> = s.split("|").collect();
            (
                pair[0]
                    .split(" ")
                    .filter(|p| !p.is_empty())
                    .map(|p| p.chars().collect())
                    .collect(),
                pair[1]
                    .split(" ")
                    .filter(|p| !p.is_empty())
                    .map(|p| p.chars().collect())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
fn day8_1(input: &Vec<(Vec<BTreeSet<char>>, Vec<BTreeSet<char>>)>) -> usize {
    input
        .iter()
        .map(|i| {
            i.1.iter()
                .filter(|p| p.len() == 2 || p.len() == 4 || p.len() == 3 || p.len() == 7)
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn day8_2(input: &Vec<(Vec<BTreeSet<char>>, Vec<BTreeSet<char>>)>) -> usize {
    input
        .iter()
        .map(|i| {
            let m: BTreeSet<BTreeSet<char>> = i.0.iter().cloned().collect();
            let all: BTreeSet<BTreeSet<char>> =
                m.union(&i.1.iter().cloned().collect()).cloned().collect();
            let mut known_patterns: HashMap<BTreeSet<char>, usize> = HashMap::new();
            let mut known_patterns_rev_lookup: HashMap<usize, BTreeSet<char>> = HashMap::new();
            let eight: Option<&BTreeSet<_>> = all.iter().find(|p| p.len() == 7);
            if let Some(eight) = eight {
                println!("found 8 {:?}", eight);
                known_patterns.insert(eight.clone(), 8);
                known_patterns_rev_lookup.insert(8, eight.clone());
            }
            let seven: Option<&BTreeSet<_>> = all.iter().find(|p| p.len() == 3);
            if let Some(seven) = seven {
                println!("found 7 {:?}", seven);
                known_patterns.insert(seven.clone(), 7);
                known_patterns_rev_lookup.insert(7, seven.clone());
            }
            let one: Option<&BTreeSet<_>> = all.iter().find(|p| p.len() == 2);
            if let Some(one) = one {
                println!("found 1 {:?}", one);
                known_patterns.insert(one.clone(), 1);
                known_patterns_rev_lookup.insert(1, one.clone());
            }
            let four: Option<&BTreeSet<_>> = all.iter().find(|p| p.len() == 4);
            if let Some(four) = four {
                println!("found 4 {:?}", four);
                known_patterns.insert(four.clone(), 4);
                known_patterns_rev_lookup.insert(4, four.clone());
            }
            let mut iters = 0;
            while known_patterns.len() < 10 {
                iters += 1;
                if iters > 1000 {
                    println!(
                        "still missing {:?}",
                        all.difference(
                            &known_patterns
                                .keys()
                                .cloned()
                                .collect::<BTreeSet<BTreeSet<char>>>()
                        )
                    );
                    panic!("could not find all numbers")
                }
                if !known_patterns_rev_lookup.contains_key(&9) {
                    let nine: Option<_> = if known_patterns_rev_lookup.contains_key(&4) {
                        all.iter().find(|p| {
                            p.len() == 6
                                && p.is_superset(known_patterns_rev_lookup.get(&4).unwrap())
                        })
                    } else if known_patterns_rev_lookup.contains_key(&0)
                        && known_patterns_rev_lookup.contains_key(&6)
                    {
                        all.iter().find(|p| {
                            p.len() == 6
                                && p.difference(known_patterns_rev_lookup.get(&0).unwrap())
                                    .count()
                                    > 0
                                && p.difference(known_patterns_rev_lookup.get(&6).unwrap())
                                    .count()
                                    > 0
                        })
                    } else {
                        Option::None
                    };
                    if let Some(nine) = nine {
                        println!("found 9 {:?}", nine);
                        known_patterns.insert(nine.clone(), 9);
                        known_patterns_rev_lookup.insert(9, nine.clone());
                    }
                }
                if !known_patterns_rev_lookup.contains_key(&0) {
                    let zero: Option<_> = if known_patterns_rev_lookup.contains_key(&8)
                        && known_patterns_rev_lookup.contains_key(&9)
                        && known_patterns_rev_lookup.contains_key(&7)
                    {
                        let desired_subset: BTreeSet<char> = known_patterns_rev_lookup
                            .get(&8)
                            .unwrap()
                            .difference(known_patterns_rev_lookup.get(&9).unwrap())
                            .cloned()
                            .collect::<BTreeSet<char>>()
                            .union(&known_patterns_rev_lookup.get(&7).unwrap().clone())
                            .cloned()
                            .collect();
                        all.iter()
                            .find(|p| p.len() == 6 && p.is_superset(&desired_subset))
                    } else if known_patterns_rev_lookup.contains_key(&6)
                        && known_patterns_rev_lookup.contains_key(&9)
                    {
                        all.iter().find(|p| {
                            p.len() == 6
                                && p.difference(known_patterns_rev_lookup.get(&6).unwrap())
                                    .count()
                                    > 0
                                && p.difference(known_patterns_rev_lookup.get(&9).unwrap())
                                    .count()
                                    > 0
                        })
                    } else {
                        Option::None
                    };
                    if let Some(zero) = zero {
                        println!("found 0 {:?}", zero);
                        known_patterns.insert(zero.clone(), 0);
                        known_patterns_rev_lookup.insert(0, zero.clone());
                    }
                }
                if !known_patterns_rev_lookup.contains_key(&6) {
                    let six: Option<_> = if known_patterns_rev_lookup.contains_key(&0)
                        && known_patterns_rev_lookup.contains_key(&9)
                    {
                        all.iter().find(|p| {
                            p.len() == 6
                                && p.difference(known_patterns_rev_lookup.get(&0).unwrap())
                                    .count()
                                    > 0
                                && p.difference(known_patterns_rev_lookup.get(&9).unwrap())
                                    .count()
                                    > 0
                        })
                    } else if known_patterns_rev_lookup.contains_key(&8)
                        && known_patterns_rev_lookup.contains_key(&2)
                        && known_patterns_rev_lookup.contains_key(&1)
                    {
                        let special_diff: BTreeSet<char> = known_patterns_rev_lookup
                            .get(&1)
                            .unwrap()
                            .intersection(&known_patterns_rev_lookup.get(&2).unwrap())
                            .cloned()
                            .collect();
                        let finding: BTreeSet<char> = known_patterns_rev_lookup
                            .get(&8)
                            .unwrap()
                            .difference(&special_diff)
                            .cloned()
                            .collect();
                        all.iter().find(|p| p.len() == 6 && p.is_superset(&finding))
                    } else {
                        Option::None
                    };
                    if let Some(six) = six {
                        println!("found 6 {:?}", six);
                        known_patterns.insert(six.clone(), 6);
                        known_patterns_rev_lookup.insert(6, six.clone());
                    }
                }
                if !known_patterns_rev_lookup.contains_key(&2) {
                    let two = if known_patterns_rev_lookup.contains_key(&8)
                        && known_patterns_rev_lookup.contains_key(&0)
                        && known_patterns_rev_lookup.contains_key(&6)
                        && known_patterns_rev_lookup.contains_key(&9)
                    {
                        let mut finding: BTreeSet<char> = known_patterns_rev_lookup
                            .get(&8)
                            .unwrap()
                            .difference(known_patterns_rev_lookup.get(&0).unwrap())
                            .cloned()
                            .collect::<BTreeSet<char>>();
                        finding.extend(
                            known_patterns_rev_lookup
                                .get(&8)
                                .unwrap()
                                .difference(known_patterns_rev_lookup.get(&6).unwrap())
                                .cloned()
                                .collect::<BTreeSet<char>>(),
                        );
                        finding.extend(
                            known_patterns_rev_lookup
                                .get(&8)
                                .unwrap()
                                .difference(known_patterns_rev_lookup.get(&9).unwrap())
                                .cloned()
                                .collect::<BTreeSet<char>>(),
                        );
                        all.iter().find(|p| p.len() == 5 && p.is_superset(&finding))
                    } else if known_patterns_rev_lookup.contains_key(&3)
                        && known_patterns_rev_lookup.contains_key(&5)
                    {
                        all.iter().find(|p| {
                            p.len() == 5
                                && !p.is_superset(known_patterns_rev_lookup.get(&5).unwrap())
                                && !p.is_superset(known_patterns_rev_lookup.get(&3).unwrap())
                        })
                    } else {
                        Option::None
                    };
                    if let Some(two) = two {
                        println!("found 2 {:?}", two);
                        known_patterns.insert(two.clone(), 2);
                        known_patterns_rev_lookup.insert(2, two.clone());
                    }
                }
                if !known_patterns_rev_lookup.contains_key(&3) {
                    let three = if known_patterns_rev_lookup.contains_key(&2)
                        && known_patterns_rev_lookup.contains_key(&5)
                    {
                        all.iter().find(|p| {
                            p.len() == 5
                                && !p.is_superset(known_patterns_rev_lookup.get(&5).unwrap())
                                && !p.is_superset(known_patterns_rev_lookup.get(&2).unwrap())
                        })
                    } else if known_patterns_rev_lookup.contains_key(&1) {
                        all.iter().find(|p| {
                            p.len() == 5
                                && p.is_superset(known_patterns_rev_lookup.get(&1).unwrap())
                        })
                    } else {
                        Option::None
                    };
                    if let Some(three) = three {
                        println!("found 3 {:?}", three);
                        known_patterns.insert(three.clone(), 3);
                        known_patterns_rev_lookup.insert(3, three.clone());
                    }
                }
                if !known_patterns_rev_lookup.contains_key(&5) {
                    let five = if known_patterns_rev_lookup.contains_key(&3)
                        && known_patterns_rev_lookup.contains_key(&2)
                    {
                        all.iter().find(|p| {
                            p.len() == 5
                                && !p.is_superset(known_patterns_rev_lookup.get(&2).unwrap())
                                && !p.is_superset(known_patterns_rev_lookup.get(&3).unwrap())
                        })
                    } else {
                        Option::None
                    };
                    if let Some(five) = five {
                        println!("found 5 {:?}", five);
                        known_patterns.insert(five.clone(), 5);
                        known_patterns_rev_lookup.insert(5, five.clone());
                    }
                }
            }
            let s: String =
                i.1.iter()
                    .map(|c| known_patterns.get(c).unwrap().to_string())
                    .collect();
            s.parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day8_1(&input), 26);
        assert_eq!(day8_2(&input), 61229);
    }
}
