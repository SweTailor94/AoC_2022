use std::fs;
use advent_of_code_2022::input::get_vector_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day3/input.txt", parse_input_line);
    println!("Day 3 part 1  (8243)");
    let sum: i32 = input.iter().map(|v| v.iter().sum::<i32>()).sum();
    println!("Sum {}", sum);
    println!("Day 3 part 2 (2631)");
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    let mut group_priorities: Vec<i32> = Vec::new();
    get_group_badge_priorities(input, &mut group_priorities);
    println!("sum {}",group_priorities.iter().sum::<i32>() );
    Ok(())
}

fn get_group_badge_priorities(input: String, group_priorities: &mut Vec<i32>) {
    let mut count = 0;
    let mut temp_group: Vec<&str> = Vec::new();
    for l in input.lines() {
        temp_group.push(l);
        count += 1;
        if count >= 3 {
            group_priorities.push(get_group_badge(&temp_group));
            temp_group.clear();
            count = 0;
        }
    }
}

fn get_group_badge(group: &Vec<&str>) -> i32 {
    for i in group[0].as_bytes(){
        if group[1].contains(*i as char)&&group[2].contains(*i as char){
            // println!("Found common badge {}", *i as char);
            return char_to_priority(*i);
        }
    }
    panic!("There should be a common item in the group!");
}

fn parse_input_line(line: &str) -> Vec<i32> {
    let pack = line.as_bytes();
    let l = pack.len() / 2;
    let mut priorities: Vec<i32> = Vec::new();
    let second_half = &pack[l..pack.len()];
    for c in &pack[0..l] {
        if second_half.contains(c) {
            priorities.push(char_to_priority(*c));
            //print!("{} ", *c as char);
            break;
        }
    }
    //println!("");
    priorities
}


fn char_to_priority(c: u8) -> i32 {
    match c {
        b'a'..=b'z' => c as i32 - 'a' as i32 + 1,
        b'A'..=b'Z' => c as i32 - 'A' as i32 + 27,
        _ => panic!("No, that( {} )cant be in an Elf rucksack!", c),
    }
}
