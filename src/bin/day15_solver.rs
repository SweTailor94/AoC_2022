use std::collections::{HashMap, HashSet};
use std::ptr::hash;
use std::time::Instant;
use itertools::Itertools;
use advent_of_code_2022::day15::day15_model::{Ranges, Sensor};
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day15/input.txt", parse_input_line);
    println!("Day 15 part 1 ");
    let y = 2000000;
    let start = Instant::now();
    let count =  1 ;//part_1(&input,y);
    let duration = start.elapsed();
    println!("Covered positions on line {} is {} and tok {} s",y,count,duration.as_secs());
    println!("Day 15 part 2 ");
    let start = Instant::now();
    let tf = part_2(&input);
    let duration = start.elapsed();
    println!("Frequency is {} and tok {} s",tf,duration.as_secs());

    Ok(())
}

fn part_1(input: &Vec<Sensor>, y:i64) -> i64 {
    let mut covered : HashSet<(i64,i64)> = HashSet::new();
    for sensor in input{
        for pos in sensor.covered_pos_on_y(y) {
            covered.insert(pos);
        }
    }
    covered.len() as i64
}

fn part_2(input: &Vec<Sensor>) -> i64{
    let mut the_world: HashMap<i64,Ranges> = HashMap::new();
    for i in 0..4000001 {
        the_world.insert(i, Ranges{covered:Vec::new()});
        if i % 100000 == 0 {
            print!(".");
        }
    }
    println!();
    let mut i = 0;
    for s in input{
        s.add_covered_in_line_ranges(&mut the_world);
        i += 1;
        println!("Worked {} sensors. Lines remaining {}",i,the_world.len());
    }
    if 1 ==  the_world.len() {
        let (row, r) =  the_world.iter().take(1).last().unwrap();
        let col = if r.covered.len() == 2 {
            r.covered[0].end
        } else if r.covered[0].start != 0{
            0
        }
        else{
            4000000
        };
        println!("({} , {})",row, col);
        return row + col * 4000000;
    }
    else{
        panic!("There should be only one left! have {}",the_world.len());
    }
}






fn parse_input_line(line:&str) -> Sensor
{
    let s_b:Vec<(i64,i64)> = line.split(':').map(|p| get_coords(p)).collect_vec();

    Sensor{
        x:s_b[0].0,
        y:s_b[0].1,
        b_x:s_b[1].0,
        b_y:s_b[1].1,
    }
}

fn get_coords(p: &str) -> (i64,i64) {
    let i = p.find("at").unwrap();
    let s = p[i+2..].split(',').collect_vec();
    (s[0].trim()[2..].parse().unwrap(),s[1].trim()[2..].parse().unwrap())
}


#[cfg(test)]
mod test {
    use std::ops::Range;
    use std::time::Instant;
    use crate::{get_coords, parse_input_line};

    #[test]
    pub fn test_coord(){
        let t = "Sensor at x=2, y=18";
        assert_eq!(get_coords(t),(2,18));
        let t = "Sensor at x=-2, y=-18";
        assert_eq!(get_coords(t),(-2,-18));
        let t = " closest beacon is at x=-2, y=15";
        assert_eq!(get_coords(t),(-2,15));
    }

    #[test]
    pub fn test_parse_line(){
        let t = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".to_string();
        let s = parse_input_line(&t);
        assert_eq!(s.x,2);
        assert_eq!(s.y,18);
        assert_eq!(s.b_x,-2);
        assert_eq!(s.b_y,15);
    }

    #[test]
    pub fn test_many(){
        let start = Instant::now();
        let mut x:i64 = 0;
        for i in 0..4000001 {
            x += 1;
        }
        let duration = start.elapsed();
        println!("{}",duration.as_millis());
        assert_eq!(x,40000001);
    }

    #[test]

    pub fn intervals() {
        let a = 1..10;
        let b = 10..12;
        assert_eq!(a.contains(&b.start),true);

    }
}
