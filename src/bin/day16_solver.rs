use std::collections::HashMap;
use itertools::Itertools;
use advent_of_code_2022::day16::day16_model::Room;
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day16/input.txt", parse_input_line);
    println!("Day 16 part 1 ");
    let mut tunnels:HashMap<String, Room> = HashMap::new();
    for i in &input{
        tunnels.insert(i.valve.clone(), i.clone());
    }
    println!("{}",part1(&tunnels));
    println!("Day 16 part 2 ");
      Ok(())
}

pub struct Step{
    pub valve:String,
    pub time_left: i32,
    pub accumulated_release: i32,
    pub from_valve: String,
}

fn part1(tunnels: &HashMap<String, Room>) -> String {


    "Hepp!".to_string()
}

fn go_to(s:Step,mut steps: Vec<Step> ,tunnels:&HashMap<String, Room>) -> Vec<Step> {

    steps
}

fn parse_input_line(line:&str) -> Room{
    // Valve AP has flow rate=0; tunnels lead to valves AA, ON

    let a = line.split(";").collect::<Vec<&str>>();
    let flow_rate = a[0].split("=").last().unwrap().parse::<i32>().unwrap();
    let leads_to = a[1][24..].split(",").map(|s|s.trim().to_string()).collect_vec();
    Room{
        valve: a[0][6..8].to_string(),
        flow_rate,
        leads_to
    }
}
