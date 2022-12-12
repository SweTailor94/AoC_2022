use advent_of_code_2022::day12::day12_model::ElfMap;
use advent_of_code_2022::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut my_map = ElfMap{
        heights: Vec::new(),
        start:(0,0),
        end:(0,0),
    };
    let input = parse_input_file("src/day12/input.txt", &mut my_map);
    println!("Day 12 part 1 ");
    println!("map is {} rows of {} columns",&my_map.heights.len(), &my_map.heights[0].len());
    println!("Start at row {} and col {}",&my_map.start.0,&my_map.start.1);
    println!("End at row {} and col {}",&my_map.end.0,&my_map.end.1);

    println!("Shortest path {}",my_map.path_finder());


    println!("\nDay 12 part 2 ");

    println!(" Shortest a to E {}",my_map.path_finder_b());
      Ok(())
}


