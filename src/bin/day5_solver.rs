use std::collections::HashMap;
use std::cmp::{min,max};
use advent_of_code_2022::input::get_vector_from_file;
use advent_of_code_2022::day5::day5_model::Line;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input: Vec::<Line> = get_vector_from_file("src/day5/input.txt", parse_input_line);
    println!("Day 5 part 1 ");
    println!("Number of lines in input {}",input.len());
    input = input.into_iter().filter(|f| f.x1 != 0 || f.y1 != 0 || f.x2 != 0 || f.y2 != 0 ).collect();
    println!("Number of horisontal or vertical lines {}",input.len());
    let mut crossings_map : HashMap<(i32,i32),i32> = HashMap::new();

    build_map(&input, &mut crossings_map);
    let count = crossings_map.values().fold(0, |sum, c| if *c >= 2 { sum + 1} else {sum});

    println!("The number of  overlapped points are {}", count );

    println!("\nDay 5 part 2 ");

    let mut crossings_p2 : HashMap<(i32,i32),i32> = HashMap::new();
    build_map_part_2(&input, &mut crossings_p2);
    let count = crossings_p2.values().fold(0, |sum, c| if *c >= 2 { sum + 1} else {sum});
    println!("The number of  overlapped points are {}", count );

    Ok(())
}
fn build_map(input: &Vec<Line>, crossings: &mut HashMap<(i32, i32), i32>) {
    for l in input {
        if l.is_vertical() {
            let x = l.x1;
            for y in min(l.y1, l.y2)..=max(l.y1, l.y2) {
                *(crossings.entry((x, y)).or_default()) += 1;
            }
        } else if l.is_horizontal() {
            let y = l.y1;
            for x in min(l.x1, l.x2)..=max(l.x1, l.x2) {
                *(crossings.entry((x, y)).or_default()) += 1;
            }
        }
    }
}

fn build_map_part_2(input: &Vec<Line>, crossings: &mut HashMap<(i32, i32), i32>) {
    for l in input{
        if l.is_vertical() {
            let x = l.x1;
            for y in min(l.y1,l.y2)..=max(l.y1,l.y2){
                *(crossings.entry((x,y)).or_default())  += 1;
            }
        }
        else if l.is_horizontal() {
            let y = l.y1;
            for x in min(l.x1, l.x2)..=max(l.x1, l.x2) {
                *(crossings.entry((x,y)).or_default()) += 1;
            }
        } else {
            if (l.x2-l.x1).abs() != (l.y2-l.y1).abs() { panic!("input is not in promised format");}
            //Diagonally
            let x_step = if l.x1 < l.x2 { 1 } else { -1 };
            let y_step = if l.y1 < l.y2 { 1 } else { -1 };
            let mut x = l.x1;
            let mut y = l.y1;
            while (x - l.x2).abs() != 0 {
                *(crossings.entry((x,y)).or_default()) += 1;
                x += x_step;
                y += y_step;
            }
            *(crossings.entry((x,y)).or_default()) += 1;
        }
    }
}



fn parse_input_line(line:&str) -> Line {
    Line::from_string(&line.to_string()).unwrap()
}
