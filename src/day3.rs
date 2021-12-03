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

pub fn filter_it(common: usize, pos: usize, input: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    input
        .iter()
        .filter(|&v| v[pos] == common)
        .map(|v| v.clone())
        .collect()
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
    let mut oxy = input.clone();
    let mut oxy_pos = 0;
    while oxy.len() > 1 {
        let r = oxy
            .iter()
            .fold(vec![0; input[0].len()], |a, b| sum_to(&a, b))
            .to_vec();
        let c = oxy.len() / 2;
        let common_value = if r[oxy_pos] > c {
            1
        } else if oxy.len() % 2 == 0 && r[oxy_pos] == c {
            1
        } else {
            0
        };
        oxy = filter_it(common_value, oxy_pos, &oxy);
        oxy_pos += 1;
    }
    let mut co2 = input.clone();
    let mut co2_pos = 0;
    while co2.len() > 1 {
        let r = co2
            .iter()
            .fold(vec![0; input[0].len()], |a, b| sum_to(&a, b))
            .to_vec();
        let c = co2.len() / 2;
        let common_value = if r[co2_pos] > c {
            0
        } else if co2.len() % 2 == 0 && r[co2_pos] == c {
            0
        } else {
            1
        };
        co2 = filter_it(common_value, co2_pos, &co2);
        co2_pos += 1;
    }
    let mut oxy_ans = 0;
    let mut co2_ans = 0;
    for x in 0..(input[0].len()) {
        let pos = input[0].len() - x - 1;
        let o = oxy[0][x];
        let c = co2[0][x];
        if o > 0 {
            oxy_ans += 2usize.pow(pos as u32);
        }
        if c > 0 {
            co2_ans += 2usize.pow(pos as u32);
        }
    }
    oxy_ans * co2_ans
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
        assert_eq!(day3_2(&input), 230);
    }
}
