use crate::day2::Direction::{DOWN, FORWARD, UP};

enum Direction {
    UP(usize),
    DOWN(usize),
    FORWARD(usize),
}

#[aoc_generator(day2)]
fn to_vec(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|i| {
            let move_str: Vec<_> = i.split(" ").collect();
            let move_n: usize = move_str.get(1).unwrap().parse().unwrap();
            match move_str.get(0).unwrap().as_ref() {
                "forward" => FORWARD(move_n),
                "up" => UP(move_n),
                "down" => DOWN(move_n),
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn day2_1(input: &Vec<Direction>) -> usize {
    let mut h = 0;
    let mut v = 0;
    for x in input {
        match x {
            FORWARD(n) => h += n,
            UP(n) => v -= n,
            DOWN(n) => v += n,
        }
    }
    h * v
}

#[aoc(day2, part2)]
fn day2_2(input: &Vec<Direction>) -> usize {
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;
    for x in input {
        match x {
            FORWARD(n) => {
                h += n;
                v += aim * n;
            }
            UP(n) => aim -= n,
            DOWN(n) => aim += n,
        }
    }
    h * v
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day2_1(&input), 150);
        assert_eq!(day2_2(&input), 900);
    }
}
