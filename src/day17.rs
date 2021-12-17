use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    x_velocity: i32,
    y_velocity: i32,
}

impl Point {
    
    fn new(x_velocity: i32, y_velocity: i32) -> Point {
        Point {
            x: 0, 
            y: 0,
            x_velocity,
            y_velocity
        }
    }
    
    fn next_step_mut(&mut self) {
            self.x = self.x + self.x_velocity;
            self.y = self.y + self.y_velocity;
            self.x_velocity += if self.x_velocity < 0 { 1 } else if self.x_velocity > 0 { -1 } else { 0 };
            self.y_velocity -= 1;
    }
    
    fn in_area(&self, x_max: i32, y_max: i32, x_min: i32, y_min: i32) -> bool {
        self.x >= x_min && self.x <= x_max && self.y >= y_min && self.y <= y_max
    }
    
    fn still_possible(&self, x_max: i32, _y_max: i32, _x_min: i32, y_min: i32) -> bool {
        if self.y < y_min && self.y_velocity <= 0 {
            false
        } else if self.x_velocity > 0 && self.x > x_max {
            false
        } else {
            true
        }
    }
    
}

fn to_vec(input: &str) -> ((i32, i32), (i32, i32)) {
    let x_start = input.find("x=").unwrap();
    let x_end = input.find(",").unwrap();
    let v = &input[x_start+2..x_end]; 
    let xs: Vec<i32> = v.split("..").filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
    let y_start = input.find("y=").unwrap();
    let ys: Vec<i32> = input[y_start+2..].split("..").filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
    ((xs[0], xs[1]), (ys[0], ys[1]))
}

fn valid_x_range(min_x: i32, max_x: i32) -> (i32, i32) {
    let mut x_inc = 1;
    while (((x_inc * x_inc) + x_inc) / 2) < min_x {
        x_inc += 1;
    }
    (x_inc, max_x)
}

#[aoc(day17, part1)]
fn day17_1(input: &str) -> usize {
    let ((x_min, x_max), (y_min, y_max)) = to_vec(input);
    let (x_start, x_end) =  valid_x_range(x_min, x_max);
    let mut maximum_y_hight = 0;
    for x in x_start..x_end {
        for y in 1..10000 {
            let mut p = Point::new(x, y);
            while p.still_possible(x_max, y_max, x_min, y_min) {
                p.next_step_mut();
                if p.in_area(x_max, y_max, x_min, y_min) {
                    maximum_y_hight = maximum_y_hight.max(((y * y) + y) / 2);
                    break;
                }
            }
        }
    }
    maximum_y_hight as usize
}

#[aoc(day17, part2)]
fn day17_2(input: &str) -> usize {
    let ((x_min, x_max), (y_min, y_max)) = to_vec(input);
    let (x_start, x_end) =  valid_x_range(x_min, x_max);
    let mut set = HashSet::new();
    for x in x_start..=x_end {
        for y in y_min..10000 {
            let mut p = Point::new(x, y);
            while p.still_possible(x_max, y_max, x_min, y_min) {
                p.next_step_mut();
                if p.in_area(x_max, y_max, x_min, y_min) {
                    set.insert((x, y));
                }
            }
        }
    }
    set.len()
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_input() {
        assert_eq!(day17_1(&TEST_INPUT), 45);
        assert_eq!(day17_2(&TEST_INPUT), 112);
    }

}
