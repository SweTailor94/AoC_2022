use std::fs;
use advent_of_code_2022::day6::day6_model::part1;
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = fs::read_to_string("src/day6/input.txt").unwrap() ;

        //get_vector_from_file("src/day6/input.txt", parse_input_line);
    println!("Day 6 part 1 ");
    println!("{}", part1(&input,4));
    println!("Day 6 part 2 ");
    println!("{}", part1(&input,14));
      Ok(())
}



fn parse_input_line(line:&str) -> usize{
    line.len()
}
