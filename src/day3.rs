#[aoc_generator(day3)]
fn to_vec(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|i| {
            i.split("")
                .filter(|&i| !i.is_empty())
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn sum_to(xs: &Vec<usize>, ys: &Vec<usize>) -> Vec<usize> {
    let mut zs = xs.clone();
    for x in 0..(xs.len()) {
        zs[x] += ys[x];
    }
    zs
}

#[aoc(day3, part1)]
fn day3_1(input: &Vec<Vec<usize>>) -> usize {
    let r = input
        .iter()
        .fold(vec![0; input[0].len()], |a, b| sum_to(&a, b))
        .to_vec();
    let l = input.len() / 2;
    let mut gamma = 0;
    let mut eps = 0;
    for x in 0..(r.len()) {
        let pos = r.len() - x - 1;
        let v = r[x];
        if v > l {
            gamma += 2usize.pow(pos as u32);
        } else {
            eps += 2usize.pow(pos as u32);
        }
    }
    gamma * eps
}

#[aoc(day3, part2)]
fn day3_2(input: &Vec<Vec<usize>>) -> usize {
    //wip
    let r = input
        .iter()
        .fold(vec![0; input[0].len()], |a, b| sum_to(&a, b))
        .to_vec();
    let l = input.len() / 2;
    let mut gamma = 0;
    let mut eps = 0;
    for x in 0..(r.len()) {
        let pos = r.len() - x - 1;
        let v = r[x];
        if v > l {
            gamma += 2usize.pow(pos as u32);
        } else {
            eps += 2usize.pow(pos as u32);
        }
    }
    gamma * eps
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day3_1(&input), 198);
    }
}
