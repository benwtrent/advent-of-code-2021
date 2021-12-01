

#[aoc_generator(day1)]
fn to_vec(input: &str) -> Vec<usize> {
    input.lines().map(|i| i.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn day1_1(input: &Vec<usize>) -> usize {
    let mut p = input[0];
    let mut count = 0;
    for &x in input[1..].iter() {
        if x > p {
            count += 1;
        }
        p = x
    }
    count
}

#[aoc(day1, part2)]
fn day1_2(input: &Vec<usize>) -> usize {
    let mut v = [].to_vec();
    for x in input.windows(3) {
        v.push(x.iter().fold(0usize, |a,&b| a + b));
    }
    day1_1(&v)
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day1_1(&input), 7);
        assert_eq!(day1_2(&input), 5);
    }
}
