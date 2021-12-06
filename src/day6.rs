#[aoc_generator(day6)]
fn to_vec(input: &str) -> Vec<usize> {
    input
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|i| i.parse().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn day6_1(input: &Vec<usize>) -> usize {
    let mut fishes = input.clone();
    let mut created_fishes = vec![];
    for _ in 0..=8 {
        let mut to_create = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                to_create += 1;
                *fish = 6
            } else {
                *fish -= 1;
            }
        }
        created_fishes.push(to_create)
    }
    for _ in 9..80 {
        let d6 = created_fishes[created_fishes.len() - 7];
        let d8 = created_fishes[created_fishes.len() - 9];
        created_fishes.push(d6 + d8)
    }
    let s: usize = created_fishes.iter().sum();
    s + input.len()
}

#[aoc(day6, part2)]
fn day6_2(input: &Vec<usize>) -> usize {
    let mut fishes = input.clone();
    let mut created_fishes = vec![];
    for _ in 0..=8 {
        let mut to_create = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                to_create += 1;
                *fish = 6
            } else {
                *fish -= 1;
            }
        }
        created_fishes.push(to_create)
    }
    for _ in 9..256 {
        let d6 = created_fishes[created_fishes.len() - 7];
        let d8 = created_fishes[created_fishes.len() - 9];
        created_fishes.push(d6 + d8)
    }
    let s: usize = created_fishes.iter().sum();
    s + input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_input() {
        let input = to_vec(TEST_INPUT);
        assert_eq!(day6_1(&input), 5934);
        assert_eq!(day6_2(&input), 26984457539);
    }
}
