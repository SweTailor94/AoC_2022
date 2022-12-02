use advent_of_code_2022::input::get_vector_from_file;


fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day2/input.txt", parse_input_line_1);
    println!("Day 2 part 1 ");
     let score:i32 = input.iter().sum();
    println!("Score {}",score);
    println!("\nDay 2 part 2 ");
    let input = get_vector_from_file("src/day2/input.txt", parse_input_line_2);
    let score:i32 = input.iter().sum();
    println!("Score {}",score);

    Ok(())
}

fn parse_input_line_1(line:&str) -> i32 {
// A , X Rock       1 p
// B , Y Paper      2 p
// C , Z Scissors   3 p

//  Lost            0 p
//  Draw            3 p
//  Win             6 p

    match line {
        "A X" => 1 +3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        _ => 0 // Not valid input
    }
}

fn parse_input_line_2(line:&str) -> i32 {
// A  Rock       1 p
// B  Paper      2 p
// C  Scissors   3 p
// X    I shall loose
// Y    draw
// Z    I shall win
//  Lost            0 p
//  Draw            3 p
//  Win             6 p

    match line {
        "A X" => 3 + 0, // select C
        "A Y" => 1 + 3, // A
        "A Z" => 2 + 6, // B

        "B X" => 1 + 0, // A
        "B Y" => 2 + 3, // B
        "B Z" => 3 + 6, // C

        "C X" => 2 + 0, // B
        "C Y" => 3 + 3, // C
        "C Z" => 1 + 6, // A
        _ => 0 // Not valid input
    }
}