use itertools::Itertools;
use advent_of_code_2022::day18::day18_model::{another_try, Droplet, surface_area};
use advent_of_code_2022::input::{get_vector_from_file, parse_input_file, parse_input_string};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day18/input.txt", parse_input_line);
    println!("Day 18 part 1 ");
    // println!("free sides {}",surface_area(input));
    println!("another try {}", another_try(input) );
    println!("Day 18 part 2 ");
    let mut droplet = Droplet::new();
    let _ = parse_input_file("src/day18/input.txt", &mut droplet);
    println!("Exterior area is {}",droplet.exterior_surface_area());
    Ok(())
}

fn parse_input_line(line:&str) -> (i32,i32,i32){

    let xyz = line.split(",").map(|v|v.parse::<i32>().unwrap()).collect_vec();
    (xyz[0],xyz[1],xyz[2])
}
