#[aoc_generator(day7)]
fn to_vec(input: &str) -> Vec<i32> {
    input
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|i| i.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn day7_1(input: &Vec<i32>) -> i32 {
    let min_v = *input.iter().min().unwrap() as usize;
    let max_v = *input.iter().max().unwrap() as usize;
    let mut min_diff = std::i32::MAX;
    for n in min_v..max_v {
        let n = n as i32;
        min_diff = min_diff.min(input.iter().map(|i| ((*i as i32 - n as i32).abs())).sum())
    }
    min_diff
}

#[aoc(day7, part2)]
fn day7_2(input: &Vec<i32>) -> i32 {
    let min_v = *input.iter().min().unwrap() as usize;
    let max_v = *input.iter().max().unwrap() as usize;
    let mut min_diff = std::i32::MAX;
    for n in min_v..max_v {
        let n = n as i32;
        min_diff = min_diff.min(
            input
                .iter()
                .map(|i| ((*i as i32 - n as i32).abs()))
                .map(|i| (i.pow(2) + i) / 2)
                .sum(),
        )
    }
    min_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day7_1(&input), 37);
        assert_eq!(day7_2(&input), 168);
    }
}
