use std::fmt::{Display, Formatter};
use advent_of_code_2022::input::get_vector_from_file;


fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input = get_vector_from_file("src/day8/input.txt", parse_input_line);
    println!("Day 8 part 1 ");
    set_visible_from_rows(&mut input);
    // print_trees(&input);
    println!();
    let mut trans_input = transpose(&input);
    set_visible_from_rows(&mut trans_input);
    // print_trees(&transpose(&trans_input));
    println!("Visible {}", count_visible(&trans_input));

    println!("Day 8 part 2 ");
    println!("Higest score {}", find_higest_scenic_score(&input));
      Ok(())
}

fn print_trees(trees: &Vec<Vec<Tree>>) {
    for row in trees {
        for col in row {
            print!("{}",col)
        }
        println!();
    }
}

#[derive(Clone)]
pub struct Tree {
    pub height: u32,
    pub visible: bool,
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.visible {
            write!(f,"[{}]",self.height)
        }else{
            write!(f," {} ",self.height)
        }
    }
}

fn parse_input_line(line:&str) -> Vec<Tree>
{
    let mut row = Vec::new();
    for c in line.chars(){
       row.push( Tree{ height: c.to_digit(10).unwrap(), visible: false});
    }
    row
}

fn transpose(mat : &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    let mut result = Vec::new();
    let in_line_len = mat[0].len();
    for col in 0..in_line_len {
        let mut reslut_line = Vec::new();
        for i in 0..mat.len() {
            reslut_line.push(mat[i][col].clone());
        }
        result.push(reslut_line);
    }
    result
}

fn set_visible_from_rows(input : &mut Vec<Vec<Tree>>) {

    for row in input{
        let row_len =row.len();
        row[0].visible = true;
        let mut highest = row[0].height;
        for i in 1..(row_len-1){
            if row[i-1].height < row[i].height &&
               highest < row[i].height {
                // row[i] visible
                row[i].visible = true;
            }
            if row[i-1].height > highest {
                highest = row[i-1].height
            }
        }
        let start = row_len-1;
        row[start].visible = true;
        let mut highest = row[start].height;
        for i in 1..(row_len-1){
            let index = start-i;
            if row[index+1].height < row[index].height &&
                highest < row[index].height {
                row[index].visible = true;
            }
            if row[index+1].height > highest {
                highest = row[index+1].height
            }
        }
    }
}

pub fn count_visible(input: &Vec<Vec<Tree>>) -> u32{
    let mut sum = 0;
    input.iter().map(|v| v.iter().map(|t| if t.visible {1}else {0} ).sum::<u32>()).sum()
}

pub fn find_higest_scenic_score(input: &Vec<Vec<Tree>>) -> i32 {
    let rows = input.len();
    let cols = input[0].len();

    let score = |row: usize, col: usize, height: u32| {
        let mut score_left = 1;
        let mut score_right = 1;
        let mut score_up = 1;
        let mut score_down = 1;
        let mut index = (col - 1 )as i32;
        println!();
        println!("row {} col {} height {}",row,col,height);
        println!("Left: ");
        print!("{}, ",input[row][index as usize].height);
        while index > 0 && input[row][index as usize].height < height {
            print!("{}, ",input[row][index as usize].height);
            score_left += 1;
            index -= 1;
        }
        println!();
        index = (col + 1) as i32;
        println!("Right: ");
        print!("{}, ",input[row][index as usize].height);
        while index < (cols - 1) as i32 && input[row][index as usize].height < height {
            print!("{} ",input[row][index as usize].height);
            score_right += 1;
            index += 1;
        }
        println!();
        index = (row - 1) as i32;
        println!("Up: ");
        print!("{}, ",input[index as usize][col].height);
        while index > 0 && input[index as usize][col].height < height {
            print!("{}, ",input[index as usize][col].height);
            score_up += 1;
            index -= 1;
        }
        println!();
        index = (row +1) as i32;
        println!("Down: ");
        print!("{}, ",input[index as usize][col].height);
        while index < (rows-1) as i32 && input[index as usize][col].height < height {
            print!("{}, ",input[index as usize][col].height);
            score_down += 1;
            index +=1;
        }
        println!();
        println!("Score ({} * {} * {} * {}",score_left,score_right,score_up,score_down);
        score_right * score_left * score_up * score_down
    };

    let mut max = 0;
    for row in 1..(rows-1) {
        for col in 1..(cols-1) {
            let score = score(row,col,input[row][col].height);
            if score > max {
                max = score;
            }
        }
    }
    max
}

