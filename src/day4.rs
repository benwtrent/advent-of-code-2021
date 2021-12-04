use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Cube {
    board: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
    contains: HashSet<usize>,
    seen: HashSet<usize>,
    pub bingoed: usize,
}

impl From<&str> for Cube {
    fn from(s: &str) -> Self {
        let board: Vec<Vec<usize>> = s
            .split("\n")
            .map(|l| {
                l.split(" ")
                    .filter(|f| !f.is_empty())
                    .map(|i| i.parse().unwrap())
                    .collect()
            })
            .collect();
        let contains: HashSet<usize> = board.iter().flatten().map(|&n| n).collect();
        let seen = HashSet::new();
        let marked = vec![vec![false; 5]; 5];
        let bingoed = 0;
        Cube {
            board,
            marked,
            contains,
            seen,
            bingoed,
        }
    }
}

impl Cube {
    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    sum += self.board[i][j]
                }
            }
        }
        sum
    }

    fn bingo(&mut self, iteration: usize) {
        if self.bingoed > 0 {
            return;
        }
        for row in self.marked.iter() {
            if row.iter().all(|&b| b) {
                self.bingoed = iteration;
            }
        }
        for i in 0..5 {
            if self.marked.iter().map(|col| col[i]).all(|b| b) {
                self.bingoed = iteration;
            }
        }
    }

    fn mark(&mut self, n: usize) {
        if self.bingoed > 0 {
            return;
        }
        if self.contains.contains(&n) && self.seen.insert(n) {
            for r in 0..5 {
                for c in 0..5 {
                    if self.board[r][c] == n {
                        self.marked[r][c] = true
                    }
                }
            }
        }
    }
}

#[aoc_generator(day4)]
fn to_vec(input: &str) -> (Vec<usize>, Vec<Cube>) {
    let inputs: Vec<&str> = input.split("\n\n").filter(|s| !s.is_empty()).collect();
    let nums: Vec<usize> = inputs[0].split(",").map(|s| s.parse().unwrap()).collect();
    let boards = inputs[1..].iter().map(|&s| s.into()).collect();
    (nums, boards)
}

#[aoc(day4, part1)]
fn day4_1(input: &(Vec<usize>, Vec<Cube>)) -> usize {
    let nums = &input.0;
    let mut cubes = input.1.clone();
    for (i, &n) in nums.iter().enumerate() {
        for cube in cubes.iter_mut() {
            cube.mark(n);
            cube.bingo(i)
        }
    }
    let first_matched = cubes
        .iter()
        .min_by(|x, y| x.bingoed.cmp(&y.bingoed))
        .unwrap();
    first_matched.sum_unmarked() * nums[first_matched.bingoed]
}

#[aoc(day4, part2)]
fn day4_2(input: &(Vec<usize>, Vec<Cube>)) -> usize {
    let nums = &input.0;
    let mut cubes = input.1.clone();
    for (i, &n) in nums.iter().enumerate() {
        for cube in cubes.iter_mut() {
            cube.mark(n);
            cube.bingo(i);
        }
    }
    let last_matched = cubes
        .iter()
        .max_by(|x, y| x.bingoed.cmp(&y.bingoed))
        .unwrap();
    last_matched.sum_unmarked() * nums[last_matched.bingoed]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day4_1(&input), 4512);
        assert_eq!(day4_2(&input), 1924);
    }
}
