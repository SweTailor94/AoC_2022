use std::borrow::Cow;
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {



    let input = get_vector_from_file("src/day6/input.txt", parse_input_line);
    let mut all_lanternfish = input[0].clone();
    println!("Day 6 part 1 ");
    println!("Start nr of fish {}",all_lanternfish.len());

    let nr_of_days = 80;
    for day in 0..nr_of_days{
        let mut new_ones: Vec<u8> = Vec::new();
        for i in 0..all_lanternfish.len() {
            match all_lanternfish[i] {
                0 => {
                    all_lanternfish[i] = 6;
                    new_ones.push(8);
                }
                _ => all_lanternfish[i] -= 1,
            }
        }
        all_lanternfish.append(&mut new_ones);
    }

    println!("After {} days there are {} lanternfish ", nr_of_days, all_lanternfish.len() );

    println!("Day 6 part 2 ");
    let nr_of_days = 256;
    let mut count_generations: Vec<u64>  = vec![0_u64 ,0,0,0,0,0,0,0,0];
    for i in &input[0]{
        count_generations[*i as usize] += 1;
    }
    print_generations(& count_generations, &0);

    let mut day_0 = 0;
    for _day in 0..nr_of_days {
        let add_to_6 = count_generations[day_0];
        day_0 = (day_0 + 1) % 9;
        let day_6_index = (day_0 + 6) % 9;
        count_generations[day_6_index] += add_to_6;

        println!("-- After day {} --",_day+1);
        print_generations(& count_generations, &day_0);
    }
    let total:u64 = count_generations.iter().sum();

    println!("After {} days there are {} lanternfish ", nr_of_days, total );

      Ok(())
}

fn print_generations(count_generations: & Vec<u64>, day_0: &usize) {
    let mut i = *day_0;
    let mut gen = 0;
    for _ in 0..9  {
        println!("in gen {} : {} fish",gen, count_generations[i]);
        gen = (gen + 1);
        i = (i+1)%9;

    }
}

fn parse_input_line(line:&str) -> Vec<u8>{
    let fish:Vec<u8> = line.split(",").map(|s|s.parse::<u8>().unwrap()).collect();
    fish
}

//144706892713
// 26984457539
// 26984457539