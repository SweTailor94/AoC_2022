use itertools::Itertools;
use advent_of_code_2022::day10::day10_model::{Computer, Op};
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day10/input.txt", parse_input_line);
    println!("Day 10 part 1 ");
    let mut my_computer = Computer::new(input);
    let signal:i32 = (20..=220).step_by(40).map(|c| my_computer.run_until_cycle(c)).sum();
    println!("c: {}",signal);
    println!("\nDay 10 part 2 ");
    let input = get_vector_from_file("src/day10/input.txt", parse_input_line);
    let mut my_computer = Computer::new(input);

    loop{
        match my_computer.run_until_cycle2(40) {
            None => {break;}
            Some(line) => {
                println!("{}",line);
            }
        }
    }


    Ok(())
}



fn parse_input_line(line:&str) -> Op{
    let instr = line.split(' ').collect_vec();
    match instr[0] {
        "noop" => Op::Nop,
        "addx" => Op::Addx(instr[1].parse().unwrap() ),
        _ => panic!("Parse error"),
    }
}
