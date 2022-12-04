use advent_of_code_2022::day4::day4_model::AssignmentPair;
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day4/input.txt", parse_input_line);
    println!("Day 4 part 1 ");

    let sum = &input.iter().fold(0,|sum,a| if a.is_one_contained() { sum + 1 } else { sum } );
    println!("Count {}",sum);

    println!("\nDay 4 part 2 ");
    let sum = &input.iter().fold(0,|sum,a| if a.is_overlapping() { sum + 1 } else { sum } );
    println!("Count {}",sum);

    Ok(())
}

fn parse_input_line(line:&str) -> AssignmentPair {

    AssignmentPair::new(line)
}
