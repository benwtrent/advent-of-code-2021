use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

#[aoc_generator(day10)]
fn to_vec(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day10, part1)]
fn day10_1(input: &Vec<Vec<char>>) -> usize {
    let scores: HashMap<char, usize> =
        HashMap::<_, _>::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    input
        .iter()
        .map(|i| {
            let mut char_stack = vec![];
            let mut score = 0;
            for &c in i {
                if set!['{', '(', '<', '['].contains(&c) {
                    char_stack.push(c);
                } else {
                    if let Some(opening) = char_stack.pop() {
                        match opening {
                            '{' => {
                                if c != '}' {
                                    score = *scores.get(&c).unwrap();
                                    break;
                                }
                            }
                            '(' => {
                                if c != ')' {
                                    score = *scores.get(&c).unwrap();
                                    break;
                                }
                            }
                            '<' => {
                                if c != '>' {
                                    score = *scores.get(&c).unwrap();
                                    break;
                                }
                            }
                            '[' => {
                                if c != ']' {
                                    score = *scores.get(&c).unwrap();
                                }
                            }
                            _ => panic!("foo"),
                        }
                    } else {
                        score = *scores.get(&c).unwrap();
                        break;
                    }
                }
            }
            score
        })
        .sum()
}

#[aoc(day10, part2)]
fn day10_2(input: &Vec<Vec<char>>) -> usize {
    let incomplete_lines: Vec<&Vec<char>> = input
        .iter()
        .filter(|&i| {
            let mut char_stack = vec![];
            for &c in i {
                if set!['{', '(', '<', '['].contains(&c) {
                    char_stack.push(c);
                } else {
                    if let Some(opening) = char_stack.pop() {
                        let errored = match opening {
                            '{' => c != '}',
                            '(' => c != ')',
                            '<' => c != '>',
                            '[' => c != ']',
                            _ => panic!("foo"),
                        };
                        if errored {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .collect();
    let mut scores: Vec<usize> = incomplete_lines
        .iter()
        .map(|&i| {
            let mut char_stack = vec![];
            for &c in i {
                if set!['{', '(', '<', '['].contains(&c) {
                    char_stack.push(c);
                } else {
                    char_stack.pop();
                }
            }
            let mut score = 0;
            while !char_stack.is_empty() {
                if let Some(c) = char_stack.pop() {
                    score *= 5;
                    score += match c {
                        '{' => 3,
                        '(' => 1,
                        '<' => 4,
                        '[' => 2,
                        _ => panic!("foo"),
                    };
                }
            }
            score
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day10_1(&input), 26397);
        assert_eq!(day10_2(&input), 288957);
    }
}
