use std::collections::VecDeque;
use advent_of_code_2022::day7::day7_model::{ElfFolder, FileSystem};
use advent_of_code_2022::input::{get_vector_from_file, parse_input_file};

// This code is not finished yet ...
fn main() ->Result<(),Box<dyn std::error::Error>> {

    let mut input = FileSystem{
        root : ElfFolder{
            name : "/".to_string(),
            files : Vec::new(),
            folders: Vec::new()
        },
        cur_dir : VecDeque::new(),
    };
    let _ = parse_input_file("src/day7/input.txt", &mut input);
    println!("Day 7 part 1 ");
    println!("Day 7 part 2 ");
      Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
