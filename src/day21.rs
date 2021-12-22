use std::collections::HashMap;
use std::fmt::Debug;

#[aoc_generator(day21)]
fn to_vec(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();
    let p1slc = lines[0].replace("Player 1 starting position: ", "");
    let p2slc = lines[1].replace("Player 2 starting position: ", "");
    (p1slc.parse().unwrap(), p2slc.parse().unwrap())
}

#[aoc(day21, part1)]
fn day21_1(input: &(usize, usize)) -> usize {
    let (mut p1_pos, mut p2_pos) = input;
    let (mut p1_score, mut p2_score) = (0usize, 0usize);
    let die_rolls: Vec<usize> = (1..=100).collect();
    let mut die_index = 0usize;
    let mut number_of_rolls = 0usize;
    while true {
        p1_pos += die_rolls[die_index]
            + die_rolls[(die_index + 1) % 100]
            + die_rolls[(die_index + 2) % 100];
        p1_pos %= 10;
        if p1_pos == 0 {
            p1_pos = 10;
        }
        p1_score += p1_pos;
        number_of_rolls += 3;
        die_index += 3;
        die_index %= 100;
        if p1_score >= 1000 {
            break;
        }
        p2_pos += die_rolls[die_index]
            + die_rolls[(die_index + 1) % 100]
            + die_rolls[(die_index + 2) % 100];
        p2_pos %= 10;
        if p2_pos == 0 {
            p2_pos = 10;
        }
        p2_score += p2_pos;
        number_of_rolls += 3;
        die_index += 3;
        die_index %= 100;
        if p2_score >= 1000 {
            break;
        }
    }
    number_of_rolls * if p1_score >= 1000 { p2_score } else { p1_score }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Player {
    pos: usize,
    score: usize,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Scenario {
    p1: Player,
    p2: Player,
    next: usize,
}

#[aoc(day21, part2)]
fn day21_2(input: &(usize, usize)) -> usize {
    let (mut p1_pos, mut p2_pos) = input;
    let scenario = Scenario {
        p1: Player {
            pos: p1_pos,
            score: 0,
        },
        p2: Player {
            pos: p2_pos,
            score: 0,
        },
        next: 0,
    };
    let mut wins = [0; 2];
    let mut outcomes = HashMap::new();
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let mut n = scenario.clone();
                n.player[n.next_player].position =
                    (n.player[n.next_player].position + a + b + c - 1) % 10 + 1;
                n.player[n.next_player].score += n.player[n.next_player].position;
                if n.player[n.next_player].score >= 21 {
                    res[n.next_player] += 1;
                } else {
                    n.next_player = 1 - n.next_player;
                    let r = n.get_wins(visited);
                    res[0] += r[0];
                    res[1] += r[1];
                }
            }
        }
    }
    visited.insert(self, res);
    let (mut p1_score, mut p2_score) = (0usize, 0usize);
    let die_rolls: Vec<usize> = (1..=100).collect();
    let mut die_index = 0usize;
    let mut number_of_rolls = 0usize;
    while true {
        p1_pos += die_rolls[die_index]
            + die_rolls[(die_index + 1) % 100]
            + die_rolls[(die_index + 2) % 100];
        p1_pos %= 10;
        if p1_pos == 0 {
            p1_pos = 10;
        }
        p1_score += p1_pos;
        number_of_rolls += 3;
        die_index += 3;
        die_index %= 100;
        if p1_score >= 1000 {
            break;
        }
        p2_pos += die_rolls[die_index]
            + die_rolls[(die_index + 1) % 100]
            + die_rolls[(die_index + 2) % 100];
        p2_pos %= 10;
        if p2_pos == 0 {
            p2_pos = 10;
        }
        p2_score += p2_pos;
        number_of_rolls += 3;
        die_index += 3;
        die_index %= 100;
        if p2_score >= 1000 {
            break;
        }
    }
    number_of_rolls * if p1_score >= 1000 { p2_score } else { p1_score }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_input() {
        assert_eq!(day21_1(&to_vec(TEST_INPUT)), 739785);
        assert_eq!(day21_2(&to_vec(TEST_INPUT)), 0)
    }
}
