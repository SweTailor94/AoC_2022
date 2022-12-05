use advent_of_code_2022::day5::day5_model::Inputs;
use advent_of_code_2022::input::{get_vector_from_file, parse_input_file};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut my_input = Inputs::new();
    let input = parse_input_file("src/day5/input.txt",&mut my_input);
    println!("Day 5 part 1 ");
    let my_moves = my_input.moves;
    for m in my_moves {
        my_input.my_stacks.move_crates(m.count, m.from, m.to  );
    }
    println!("{}",my_input.my_stacks.get_top_string());

    println!("\nDay 5 part 2 ");
    let mut my_input = Inputs::new();
    let input = parse_input_file("src/day5/input.txt",&mut my_input);
    let my_moves = my_input.moves;
    for m in my_moves {
        my_input.my_stacks.move_crates_9001(m.count, m.from, m.to  );
    }
    println!("{}",my_input.my_stacks.get_top_string());
    Ok(())
}

fn parse_input_line(line:&str) -> usize{
    line.len()
}
