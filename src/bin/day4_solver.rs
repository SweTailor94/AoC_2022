use advent_of_code_2022::input::parse_input_file;
use advent_of_code_2022::day4::day4_model::{InputData};

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut my_input = InputData::new();
    let _ = parse_input_file("src/day4/input.txt", &mut my_input)?;
    println!("Day 4 part 1 ");
    println!("# of numbers in draw {}", my_input.draw.len());
    println!("# of boards {}",my_input.boards.len());
    // print_boards(& my_input);
    let (number, score, board_number) = run_until_bingo(&mut my_input);
    println!("Winning score {} at board {} last number drawn {}",score, board_number, number);
    println!("\nDay 4 part 2 ");
    let (number, score, board_number) = run_until_last_bingo(&mut my_input);
    println!("Winning score {} at board {} last number drawn {}",score, board_number, number);


      Ok(())
}

fn run_until_bingo(my_input: &mut InputData)  -> (i32, i32, i32 ){
    for n in &my_input.draw {
        //println!("-------- Nummer {} ---------", n);
        let mut board_number = 0;
        for b in &mut my_input.boards {
            board_number += 1;
            if let Some(score) = b.mark_number(*n) {
                return (*n, score, board_number);
            }
        }
        //print_boards(&my_input);
    }
    return (0,0,0,);
}
fn run_until_last_bingo(my_input: &mut InputData)  -> (i32, i32, i32 ){
    let mut bingos = 0;
    let number_of_boards = my_input.boards.len();
    let mut last_n = 0;
    let mut last_score = 0;
    let mut last_board = 0;
    for n in &my_input.draw {
        // println!("-------- Nummer {} ---------", n);
        let mut board_number = 0;
        for b in &mut my_input.boards {
            board_number += 1;
            if let Some(score) = b.mark_number(*n) {
                bingos += 1;
                println!("Bingo # {}, board {}",bingos,board_number);
                last_n = *n;
                last_score = score;
                last_board = board_number;
                if bingos == number_of_boards{
                    return (*n, score, board_number);
                }
            }
        }
        //print_boards(&my_input);
    }
    return (last_n, last_score, last_board);
}
fn print_boards(my_input: &InputData) {
    for board in &my_input.boards {
        board.print();
        println!();
    }
}


