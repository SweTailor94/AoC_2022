extern crate core;

use std::collections::HashSet;
use itertools::Itertools;
use advent_of_code_2022::input::get_vector_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day9/input.txt", parse_input_line);
    println!("Day 9 part 1 ");
    println!("positions visited {}", part1(&input));

    println!("Day 9 part 2 ");
    println!("positions visited {}", part2(&input));
    Ok(())
}

fn part1(input: &Vec<Direction>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail.clone());

    for m in input {
        match m {
            Direction::Up(steps) => {
                for _i in 0..*steps {
                    head.1 += 1;
                    if is_adjacent(&head, &tail) { continue; }
                    tail_follow(&head, &mut tail);
                    visited.insert(tail.clone());
                }
            }
            Direction::Down(steps) => {
                for _i in 0..*steps {
                    head.1 -= 1;
                    if is_adjacent(&head, &tail) { continue; }
                    tail_follow(&head, &mut tail);
                    visited.insert(tail.clone());
                }
            }
            Direction::Left(steps) => {
                for _i in 0..*steps {
                    head.0 -= 1;
                    if is_adjacent(&head, &tail) { continue; }
                    tail_follow(&head, &mut tail);
                    visited.insert(tail.clone());
                }
            }
            Direction::Right(steps) => {
                for _i in 0..*steps {
                    head.0 += 1;
                    if is_adjacent(&head, &tail) { continue; }
                    tail_follow(&head, &mut tail);
                    visited.insert(tail.clone());
                }
            }
        }
    }


    visited.len()
}

fn part2(input: &Vec<Direction>) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = [(0, 0); 10].to_vec();
    visited.insert(rope[9].clone());

    /*
    let follo_rope = || {
        for i in 0..9 {
            if is_adjacent(&rope[i], &rope[i+1]) { continue; }
            let temp = tail_follow2(&rope[i],  &rope[i+1]);
            rope[i+1].0 = temp.0;
            rope[i+1].1 = temp.1;
            visited.insert(rope[i+1].clone());
        }
    };
    */

    for m in input {
        match m {
            Direction::Up(steps) => {
                for _i in 0..*steps {
                    rope[0].1 += 1;
                    for i in 0..9 {
                        if is_adjacent(&rope[i], &rope[i + 1]) { continue; }
                        let temp = tail_follow2(&rope[i], &rope[i + 1]);
                        rope[i + 1].0 = temp.0;
                        rope[i + 1].1 = temp.1;
                    }
                    visited.insert(rope[9].clone());
                }
            }
            Direction::Down(steps) => {
                for _i in 0..*steps {
                    rope[0].1 -= 1;
                    for i in 0..9 {
                        if is_adjacent(&rope[i], &rope[i + 1]) { continue; }
                        let temp = tail_follow2(&rope[i], &rope[i + 1]);
                        rope[i + 1].0 = temp.0;
                        rope[i + 1].1 = temp.1;
                    }
                    visited.insert(rope[9].clone());
                }
            }
            Direction::Left(steps) => {
                for _i in 0..*steps {
                    rope[0].0 -= 1;
                    for i in 0..9 {
                        if is_adjacent(&rope[i], &rope[i + 1]) { continue; }
                        let temp = tail_follow2(&rope[i], &rope[i + 1]);
                        rope[i + 1].0 = temp.0;
                        rope[i + 1].1 = temp.1;
                    }
                    visited.insert(rope[9].clone());
                }
            }
            Direction::Right(steps) => {
                for _i in 0..*steps {
                    rope[0].0 += 1;
                    for i in 0..9 {
                        if is_adjacent(&rope[i], &rope[i + 1]) { continue; }
                        let temp = tail_follow2(&rope[i], &rope[i + 1]);
                        rope[i + 1].0 = temp.0;
                        rope[i + 1].1 = temp.1;
                    }
                    visited.insert(rope[9].clone());
                }
            }
        }
    }
    visited.len()
}


fn tail_follow(head: &(i32, i32), tail: &mut (i32, i32)) {
    let d = dist(head, tail);
    match d {
        (0, y) => {
            if y.abs() > 1 {
                if y > 0 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
            }
        }
        (x, 0) => {
            if x.abs() > 1 {
                if x > 0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }
            }
        }
        (x, y) => {
            if x.abs() > 1 || y.abs() > 1 {
                if y > 0 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
                if x > 0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }
            }
        }
    }
}

fn tail_follow2(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let mut tail = tail.clone();
    let d = dist(head, &tail);
    match d {
        (0, y) => {
            if y.abs() > 1 {
                if y > 0 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
            }
        }
        (x, 0) => {
            if x.abs() > 1 {
                if x > 0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }
            }
        }
        (x, y) => {
            if x.abs() > 1 || y.abs() > 1 {
                if y > 0 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
                if x > 0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }
            }
        }
    }
    tail
}

fn dist(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn is_adjacent(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let d = dist(a, b);
    d.0.abs() <= 1 && d.1.abs() <= 1
}


enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}


fn parse_input_line(line: &str) -> Direction {
    let parts = line.split(' ').collect_vec();
    match parts[0] {
        "U" => Direction::Up(parts[1].parse().unwrap()),
        "D" => Direction::Down(parts[1].parse().unwrap()),
        "L" => Direction::Left(parts[1].parse().unwrap()),
        "R" => Direction::Right(parts[1].parse().unwrap()),
        _ => panic!("Wrong type of input!"),
    }
}
