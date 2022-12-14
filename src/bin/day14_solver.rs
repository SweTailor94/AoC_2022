use std::collections::HashSet;
use advent_of_code_2022::day14::day14_model::Cave;
use advent_of_code_2022::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut my_cave: Cave = Cave{ world: HashSet::new()};
    parse_input_file("src/day14/input.txt", &mut my_cave).unwrap();
    println!("Day 14 part 1 ");
    println!("Number of units {}",my_cave.sand_part1());

    let mut my_cave: Cave = Cave{ world: HashSet::new()};
    parse_input_file("src/day14/input.txt", &mut my_cave).unwrap();
    println!("Day 14 part 2 ");
    println!("Stops at {} units",my_cave.sand_part2());
      Ok(())
}

