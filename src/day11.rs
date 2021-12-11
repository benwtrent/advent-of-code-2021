#[derive(Debug, Copy, Clone)]
struct Octopus {
    val: usize,
    num_flashes: usize,
    flashed: bool,
    pos: (usize, usize),
}

impl Octopus {
    fn new(val: usize, pos: (usize, usize)) -> Octopus {
        Octopus {
            val,
            num_flashes: 0,
            flashed: false,
            pos,
        }
    }

    fn flash(&mut self) {
        if self.flashed {
            return;
        }
        self.num_flashes += 1;
        self.flashed = true;
    }

    fn reset(&mut self) -> bool {
        if self.val > 9 {
            self.val = 0;
        }
        let ret_val = self.flashed;
        self.flashed = false;
        ret_val
    }
}

#[aoc_generator(day11)]
fn to_vec(input: &str) -> Vec<Vec<Octopus>> {
    let mut v = vec![];
    for (i, l) in input.lines().enumerate() {
        let mut o = vec![];
        for (j, v) in l.split("").filter(|s| !s.is_empty()).enumerate() {
            o.push(Octopus::new(v.parse().unwrap(), (i, j)));
        }
        v.push(o);
    }
    v
}

fn neighbors(ij: &(usize, usize), max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    let i = ij.0;
    let j = ij.1;
    let mut n = vec![];
    if i >= 1 {
        n.push((i - 1, j));
        if j >= 1 {
            n.push((i - 1, j - 1));
        }
        if j < max_j {
            n.push((i - 1, j + 1));
        }
    }
    if i < max_i {
        n.push((i + 1, j));
        if j < max_j {
            n.push((i + 1, j + 1));
        }
        if j >= 1 {
            n.push((i + 1, j - 1));
        }
    }
    if j >= 1 {
        n.push((i, j - 1));
    }
    if j < max_j {
        n.push((i, j + 1));
    }
    n
}

#[aoc(day11, part1)]
fn day11_1(input: &Vec<Vec<Octopus>>) -> usize {
    let mut octopuses = input.clone();
    let max_i = octopuses.len() - 1;
    let max_j = octopuses.first().unwrap().len() - 1;
    for _ in 0..100 {
        // inc by one
        let mut to_flash = vec![];
        for i in 0..octopuses.len() {
            for j in 0..octopuses[i].len() {
                octopuses[i][j].val += 1;
                if octopuses[i][j].val > 9 {
                    to_flash.push((i, j))
                }
            }
        }
        while !to_flash.is_empty() {
            let oct = to_flash.pop().unwrap();
            if octopuses[oct.0][oct.1].flashed {
                continue;
            }
            octopuses[oct.0][oct.1].flash();
            for n in neighbors(&oct, max_i, max_j) {
                if !octopuses[n.0][n.1].flashed {
                    octopuses[n.0][n.1].val += 1;
                    if octopuses[n.0][n.1].val > 9 {
                        to_flash.push(n)
                    }
                }
            }
        }
        for i in 0..octopuses.len() {
            for j in 0..octopuses[i].len() {
                octopuses[i][j].reset();
            }
        }
    }
    octopuses
        .iter()
        .map(|octs| octs.iter().map(|o| o.num_flashes).sum::<usize>())
        .sum()
}

#[aoc(day11, part2)]
fn day11_2(input: &Vec<Vec<Octopus>>) -> usize {
    let mut octopuses = input.clone();
    let max_i = octopuses.len() - 1;
    let max_j = octopuses.first().unwrap().len() - 1;
    for step in 1.. {
        // inc by one
        let mut to_flash = vec![];
        for i in 0..octopuses.len() {
            for j in 0..octopuses[i].len() {
                octopuses[i][j].val += 1;
                if octopuses[i][j].val > 9 {
                    to_flash.push((i, j))
                }
            }
        }
        while !to_flash.is_empty() {
            let oct = to_flash.pop().unwrap();
            if octopuses[oct.0][oct.1].flashed {
                continue;
            }
            octopuses[oct.0][oct.1].flash();
            for n in neighbors(&oct, max_i, max_j) {
                if !octopuses[n.0][n.1].flashed {
                    octopuses[n.0][n.1].val += 1;
                    if octopuses[n.0][n.1].val > 9 {
                        to_flash.push(n)
                    }
                }
            }
        }
        let mut all_flashed = true;
        for i in 0..octopuses.len() {
            for j in 0..octopuses[i].len() {
                all_flashed &= octopuses[i][j].reset();
            }
        }
        if all_flashed {
            return step;
        }
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day11_1(&input), 1656);
        assert_eq!(day11_2(&input), 195);
    }
}
