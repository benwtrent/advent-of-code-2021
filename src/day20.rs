use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Light {
    On,
    Off,
}

fn to_number(lights: &[Light]) -> usize {
    let binary_string: String = lights
        .iter()
        .map(|l| match l {
            Light::On => "1",
            Light::Off => "0",
        })
        .collect();
    usize::from_str_radix(&binary_string, 2).unwrap()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct LightAndPosition {
    state: Light,
    coordinates: [i32; 2],
}

struct CoordinateDirection2d<'a> {
    coordinates: [i32; 2],
    maximum: &'a [i32; 2],
    minimum: &'a [i32; 2],
}

impl Iterator for CoordinateDirection2d<'_> {
    type Item = [i32; 2];

    fn next(&mut self) -> Option<[i32; 2]> {
        if self.coordinates[1] > self.maximum[1] {
            return None;
        }
        let to_return = self.coordinates.clone();
        if self.coordinates[0] == self.maximum[0] {
            self.coordinates[0] = self.minimum[0];
            self.coordinates[1] += 1;
        } else {
            self.coordinates[0] += 1;
        }
        Some(to_return)
    }
}

impl From<&str> for Light {
    fn from(s: &str) -> Self {
        match s {
            "#" => Light::On,
            "." => Light::Off,
            _ => unimplemented!(),
        }
    }
}

#[aoc_generator(day20)]
fn to_vec(input: &str) -> (Vec<Light>, [i32; 2], HashMap<[i32; 2], LightAndPosition>) {
    let mut space: HashMap<[i32; 2], LightAndPosition> = HashMap::new();
    let mut y = 0;
    let mut max_x = 0;
    let things: Vec<&str> = input.split("\n\n").filter(|s| !s.is_empty()).collect();
    let enhancer: Vec<Light> = things[0]
        .split("")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.into())
        .collect();
    for l in things[1].lines() {
        for (x, s) in l.split("").filter(|&s| !s.is_empty()).enumerate() {
            space.insert(
                [x as i32, y],
                LightAndPosition {
                    state: s.into(),
                    coordinates: [x as i32, y],
                },
            );
            max_x = x.max(max_x);
        }
        if !l.trim().is_empty() {
            y += 1;
        }
    }
    (enhancer, [max_x as i32, y - 1], space)
}

#[aoc(day20, part1)]
fn day20_1(input: &(Vec<Light>, [i32; 2], HashMap<[i32; 2], LightAndPosition>)) -> usize {
    let (enhancer, max_coor, space) = input;
    let mut curr_vals = space.clone();
    let mut x_max = max_coor[0];
    let mut y_max = max_coor[1];
    let mut default_light = Light::Off;
    for _ in 0..2 {
        let mut new_vals = HashMap::new();
        x_max = x_max + 2;
        y_max = y_max + 2;
        for x in -2..=x_max {
            for y in -2..=y_max {
                let coors = [x, y];
                let lights_coors = vec![
                    [coors[0] - 1, coors[1] - 1],
                    [coors[0], coors[1] - 1],
                    [coors[0] + 1, coors[1] - 1],
                    [coors[0] - 1, coors[1]],
                    [coors[0], coors[1]],
                    [coors[0] + 1, coors[1]],
                    [coors[0] - 1, coors[1] + 1],
                    [coors[0], coors[1] + 1],
                    [coors[0] + 1, coors[1] + 1],
                ];
                let mut lights = vec![];
                for coor_iter in lights_coors.into_iter() {
                    lights.push(
                        curr_vals
                            .get(&coor_iter)
                            .map(|l| l.state)
                            .unwrap_or(default_light),
                    );
                }
                let v = to_number(lights.as_slice());
                let new_value = LightAndPosition {
                    coordinates: coors,
                    state: enhancer[v].clone(),
                };
                new_vals.insert(coors, new_value);
            }
        }
        curr_vals = new_vals;
        default_light = match default_light {
            Light::On => enhancer[enhancer.len() - 1],
            Light::Off => enhancer[0],
        };
        println!(
            "{}",
            curr_vals
                .values()
                .into_iter()
                .map(|l| match l.state {
                    Light::On => 1,
                    Light::Off => 0,
                })
                .sum::<usize>()
        );
    }
    curr_vals
        .values()
        .into_iter()
        .map(|l| match l.state {
            Light::On => 1,
            Light::Off => 0,
        })
        .sum()
}

#[aoc(day20, part2)]
fn day20_2(input: &(Vec<Light>, [i32; 2], HashMap<[i32; 2], LightAndPosition>)) -> usize {
    let (enhancer, max_coor, space) = input;
    let mut curr_vals = space.clone();
    let mut x_max = max_coor[0];
    let mut y_max = max_coor[1];
    let mut y_min = 0;
    let mut x_min = 0;
    let mut default_light = Light::Off;
    for _ in 0..50 {
        let mut new_vals = HashMap::new();
        x_min = x_min - 2;
        y_min = y_min - 2;
        x_max = x_max + 2;
        y_max = y_max + 2;
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let coors = [x, y];
                let lights_coors = vec![
                    [coors[0] - 1, coors[1] - 1],
                    [coors[0], coors[1] - 1],
                    [coors[0] + 1, coors[1] - 1],
                    [coors[0] - 1, coors[1]],
                    [coors[0], coors[1]],
                    [coors[0] + 1, coors[1]],
                    [coors[0] - 1, coors[1] + 1],
                    [coors[0], coors[1] + 1],
                    [coors[0] + 1, coors[1] + 1],
                ];
                let mut lights = vec![];
                for coor_iter in lights_coors.into_iter() {
                    lights.push(
                        curr_vals
                            .get(&coor_iter)
                            .map(|l| l.state)
                            .unwrap_or(default_light),
                    );
                }
                let v = to_number(lights.as_slice());
                let new_value = LightAndPosition {
                    coordinates: coors,
                    state: enhancer[v].clone(),
                };
                new_vals.insert(coors, new_value);
            }
        }
        curr_vals = new_vals;
        default_light = match default_light {
            Light::On => enhancer[enhancer.len() - 1],
            Light::Off => enhancer[0],
        };
    }
    curr_vals
        .values()
        .into_iter()
        .map(|l| match l.state {
            Light::On => 1,
            Light::Off => 0,
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_input() {
        assert_eq!(day20_1(&to_vec(TEST_INPUT)), 35);
        assert_eq!(day20_2(&to_vec(TEST_INPUT)), 3351)
    }
}
