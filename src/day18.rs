use std::fmt::Debug;

#[derive(Clone, Eq, PartialEq)]
enum SnailFishElement {
    Recur { values: Vec<SnailFishElement> },
    Single { value: usize },
}

impl Debug for SnailFishElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.write_to(f)
    }
}

impl From<&str> for SnailFishElement {
    //[[1,2],[[3,4],5]]
    fn from(string: &str) -> Self {
        let mut snail_stack = vec![SnailFishElement::Recur { values: vec![] }];
        let mut mut_depth = 0;
        for c in string.chars() {
            match c {
                '[' => {
                    snail_stack.push(SnailFishElement::Recur { values: vec![] });
                }
                ']' => {
                    snail_stack.pop().map(|v| {
                        if let Some(SnailFishElement::Recur { mut values }) = snail_stack.pop() {
                            values.push(v);
                            snail_stack.push(SnailFishElement::Recur { values });
                        } else {
                            panic!("ARG");
                        }
                    });
                    mut_depth -= 1;
                }
                ',' => {}
                '0'..='9' => {
                    if let Some(SnailFishElement::Recur { mut values }) = snail_stack.pop() {
                        values.push(SnailFishElement::Single {
                            value: c.to_digit(10).unwrap() as usize,
                        });
                        snail_stack.push(SnailFishElement::Recur { values });
                    } else {
                        panic!("ARG");
                    }
                }
                _ => panic!("ARG"),
            }
        }
        match snail_stack.pop().unwrap() {
            SnailFishElement::Recur { values: v } => v[0].clone(),
            _ => panic!("arg"),
        }
    }
}

impl SnailFishElement {
    fn write_to(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SnailFishElement::Recur { values } => {
                write!(f, "[").ok();
                for v in values {
                    v.write_to(f).ok();
                }
                write!(f, "],")
            }
            SnailFishElement::Single { value } => {
                write!(f, "{},", value)
            }
        }
    }

    fn add(&self, other: &SnailFishElement) -> SnailFishElement {
        let lft = self.clone();
        let rgt = other.clone();
        SnailFishElement::Recur {
            values: vec![lft, rgt],
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            SnailFishElement::Recur { values } => {
                3 * values[0].magnitude() + 2 * values[1].magnitude()
            }
            SnailFishElement::Single { value } => *value,
        }
    }

    fn split(&self, already_split: bool) -> (SnailFishElement, bool) {
        if already_split {
            return (self.clone(), already_split);
        }
        match self {
            SnailFishElement::Recur { values: vs } => {
                let mut newv = vec![];
                let mut splt = already_split;
                for i in 0..vs.len() {
                    if !splt {
                        let (el, s) = vs[i].split(splt);
                        newv.push(el);
                        splt = s;
                    } else {
                        newv.push(vs[i].clone());
                    }
                }
                (SnailFishElement::Recur { values: newv }, splt)
            }
            SnailFishElement::Single { value } => {
                if *value >= 10 {
                    let lft = value / 2;
                    let rgt = (value + 1) / 2;
                    (
                        SnailFishElement::Recur {
                            values: vec![
                                SnailFishElement::Single { value: lft },
                                SnailFishElement::Single { value: rgt },
                            ],
                        },
                        true,
                    )
                } else {
                    (SnailFishElement::Single { value: *value }, already_split)
                }
            }
        }
    }

    fn handle_explosion_lft(&mut self, value: usize) -> Option<usize> {
        match self {
            SnailFishElement::Recur { values } => values[0].handle_explosion_lft(value),
            SnailFishElement::Single { value: value0 } => {
                *value0 += value;
                None
            }
        }
    }

    fn handle_explosion_rgt(&mut self, value: usize) -> bool {
        match self {
            SnailFishElement::Recur { values } => values[1].handle_explosion_rgt(value),
            SnailFishElement::Single { value: value0 } => {
                *value0 += value;
                true
            }
        }
    }

    fn explode(
        &self,
        already_exploded: bool,
        depth: usize,
    ) -> (SnailFishElement, (Option<usize>, Option<usize>), bool) {
        match self {
            SnailFishElement::Recur { values } => {
                let left = &values[0];
                let rgt = &values[1];
                match (left, rgt) {
                    (
                        SnailFishElement::Single { value: value0 },
                        SnailFishElement::Single { value: value1 },
                    ) => {
                        if depth >= 4 {
                            (
                                SnailFishElement::Single { value: 0 },
                                (Some(*value0), Some(*value1)),
                                true,
                            )
                        } else {
                            (self.clone(), (None, None), already_exploded)
                        }
                    }
                    (
                        rec @ SnailFishElement::Recur { values: _ },
                        single @ SnailFishElement::Single { value: _ },
                    ) => {
                        let (new_v, explosion, already_exploded) =
                            rec.explode(already_exploded, depth + 1);
                        let mut new_single = single.clone();
                        match explosion {
                            (v @ _, Some(ex2)) => {
                                new_single.handle_explosion_lft(ex2);
                                (
                                    SnailFishElement::Recur {
                                        values: vec![new_v, new_single],
                                    },
                                    (v, None),
                                    already_exploded,
                                )
                            }
                            (v @ _, None) => (
                                SnailFishElement::Recur {
                                    values: vec![new_v, new_single],
                                },
                                (v, None),
                                already_exploded,
                            ),
                        }
                    }
                    (
                        single @ SnailFishElement::Single { value: _ },
                        rec @ SnailFishElement::Recur { values: _ },
                    ) => {
                        let (new_v, explosion, already_exploded) =
                            rec.explode(already_exploded, depth + 1);
                        let mut new_single = single.clone();
                        match explosion {
                            (Some(ex1), v @ _) => {
                                new_single.handle_explosion_rgt(ex1);
                                (
                                    SnailFishElement::Recur {
                                        values: vec![new_single, new_v],
                                    },
                                    (None, v),
                                    already_exploded,
                                )
                            }
                            (None, v @ _) => (
                                SnailFishElement::Recur {
                                    values: vec![new_single, new_v],
                                },
                                (None, v),
                                already_exploded,
                            ),
                        }
                    }
                    (
                        r1 @ SnailFishElement::Recur { values: _ },
                        r2 @ SnailFishElement::Recur { values: _ },
                    ) => {
                        let (mut new_r1, explosion, already_exploded) =
                            r1.explode(already_exploded, depth + 1);
                        let mut new_r2 = r2.clone();
                        let mut lft = None;
                        let mut rgt = None;
                        match explosion {
                            (vp @ _, Some(v)) => {
                                new_r2.handle_explosion_lft(v);
                                lft = vp;
                            }
                            (vp @ _, _) => lft = vp,
                        }
                        let (new_r2, explosion, already_exploded) =
                            new_r2.explode(already_exploded, depth + 1);
                        match explosion {
                            (Some(v), rp @ _) => {
                                new_r1.handle_explosion_rgt(v);
                                rgt = rp
                            }
                            (_, rp @ _) => rgt = rp,
                        }
                        (
                            SnailFishElement::Recur {
                                values: vec![new_r1, new_r2],
                            },
                            (lft, rgt),
                            already_exploded,
                        )
                    }
                }
            }
            _ => (self.clone(), (None, None), already_exploded),
        }
    }
}

fn reduce(top_element: SnailFishElement) -> SnailFishElement {
    let mut did_explode = true;
    let mut did_split = true;
    let mut fish = top_element;
    while did_split || did_explode {
        let (exploded, _, ex) = fish.explode(false, 0);
        let (splitted, splt) = exploded.split(false);
        did_explode = ex;
        did_split = splt;
        fish = splitted;
    }
    fish
}

#[aoc(day18, part1)]
fn day18_1(input: &str) -> usize {
    let snails: Vec<SnailFishElement> = input.lines().map(|l| SnailFishElement::from(l)).collect();
    let mut first = snails[0].clone();
    for s in &snails[1..] {
        first = first.add(s);
        first = reduce(first);
    }
    println!("final: {:?}", first);
    first.magnitude()
}

#[aoc(day18, part2)]
fn day18_2(input: &str) -> usize {
    let snails: Vec<SnailFishElement> = input.lines().map(|l| SnailFishElement::from(l)).collect();
    let mut max_mag = 0;
    for (index, v) in snails[0..].iter().enumerate() {
        for o in &snails[index..] {
            max_mag = max_mag.max(reduce(v.clone().add(o)).magnitude());
            max_mag = max_mag.max(reduce(o.clone().add(v)).magnitude());
        }
    }
    max_mag
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_input() {
        assert_eq!(day18_1(TEST_INPUT), 4140)
    }

    #[test]
    fn test_parse() {
        assert_eq!(day18_2(TEST_INPUT), 3993)
    }

    #[test]
    fn explode() {
        let sn = SnailFishElement::from("[[6,[5,[4,[3,2]]]],1]");
        println!("{:?}", sn);
        let (exploded, _, a) = sn.explode(false, 0);
        println!("{:?} {:?}", exploded, a);
    }

    #[test]
    fn test_reduce() {
        let sn = SnailFishElement::from(
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        );
        let result = reduce(sn);
        assert_eq!(
            result,
            SnailFishElement::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        );
        println!("{:?}", result);
    }
}
