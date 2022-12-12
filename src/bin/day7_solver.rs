use std::collections::{HashMap, VecDeque};
use advent_of_code_2022::day7::day7_alt_model::{ElfFolder, FileSystem2};
use advent_of_code_2022::input::{get_vector_from_file, parse_input_file};

// This code is not finished yet ...
fn main() ->Result<(),Box<dyn std::error::Error>> {

    let mut input = FileSystem2{
        folders : HashMap::new(),
        current_dir : "/".to_string(),
    };
    let _ = parse_input_file("src/day7/input.txt", &mut input);
    println!("Day 7 part 1 ");
    println!("#folders {}",input.folders.len());
    for k in input.folders.keys() {
        println!("{}",k);
    }
    println!(" Sum of folders {}",input.sum_folders_smaller_than(100000));

    println!("\nDay 7 part 2 ");

    let mut list = input.get_folder_sizes("/");
    let total_size = list[0];
    list.sort();
    let needed_space = 30000000 - (70000000-total_size);
    if let Some(s) = list.iter().find(|v| v >= &&needed_space ){
        println!("Remove folder sized {}",s);
    }else {
        println!("Failed");
    }

      Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
