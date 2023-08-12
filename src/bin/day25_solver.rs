use std::ops::Add;
use advent_of_code_2022::day25::day25_model::Snafu;
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day25/input.txt", parse_input_line);
    println!("Day 25 part 1 ");

    let s= Snafu::new("0");
    let sum = input.iter().fold( s,|sum,x|sum.add(x));
    println!("Sum {} dec:{}",sum.to_string(),sum.dec);
    println!("Day 25 part 2 ");
      Ok(())
}

fn parse_input_line(line:&str) -> Snafu
{
    Snafu::new(line)
}
