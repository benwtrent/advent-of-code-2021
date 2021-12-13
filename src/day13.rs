use std::collections::BTreeSet;

fn to_vec(input: &str) -> (BTreeSet<(usize, usize)>, Vec<(bool, usize)>) {
    let mut splt = input.split("\n\n");
    let pts = splt
        .next()
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let p: Vec<&str> = s.split(",").filter(|ss| !ss.is_empty()).collect();
            (p[0].parse().unwrap(), p[1].parse().unwrap())
        })
        .collect();
    let instructions = splt
        .next()
        .unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let p: Vec<&str> = s.split(" ").filter(|ss| !ss.is_empty()).collect();
            let p: Vec<&str> = p[2].split("=").filter(|ss| !ss.is_empty()).collect();
            (p[0] == "y", p[1].parse().unwrap())
        })
        .collect();

    (pts, instructions)
}

fn fold(pts: &BTreeSet<(usize, usize)>, is_y: &bool, pos: &usize) -> BTreeSet<(usize, usize)> {
    let mut new_points = BTreeSet::new();
    for pt in pts {
        if *is_y {
            if pt.1 < *pos {
                new_points.insert(pt.clone());
            } else {
                new_points.insert((pt.0, *pos - (pt.1 - *pos)));
            }
        } else {
            if pt.0 < *pos {
                new_points.insert(pt.clone());
            } else {
                new_points.insert((*pos - (pt.0 - *pos), pt.1));
            }
        }
    }
    new_points
}

#[aoc(day13, part1)]
fn day13_1(input: &str) -> usize {
    let (pts, instructions) = to_vec(input);
    let first_instruction = instructions[0];
    fold(&pts, &first_instruction.0, &first_instruction.1).len()
}

#[aoc(day13, part2)]
fn day13_2(input: &str) -> usize {
    let (pts, instructions) = to_vec(input);
    let mut pts = pts;
    for instruction in instructions {
        let new_points = fold(&pts, &instruction.0, &instruction.1);
        pts = new_points;
    }
    let max_x = pts.iter().map(|p| p.0).max().unwrap();
    let max_y = pts.iter().map(|p| p.1).max().unwrap();
    for y in 0..=max_y {
        let s: String = (0..=max_x)
            .into_iter()
            .map(|x| if pts.contains(&(x, y)) { "#" } else { "." })
            .collect();
        println!("{}", s)
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_input() {
        assert_eq!(day13_1(&TEST_INPUT), 17);
        assert_eq!(day13_2(&TEST_INPUT), 17);
    }
}
