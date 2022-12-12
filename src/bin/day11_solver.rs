use itertools::Itertools;
use advent_of_code_2022::day11::day11_model::MonkeyParser;
use advent_of_code_2022::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    //let input = get_vector_from_file("src/day11/input.txt", parse_input_line);
    let mut monkey_life = MonkeyParser::new();
    parse_input_file("src/day11/input.txt", &mut monkey_life).unwrap();

    monkey_life.set_common_devisor();

    println!("Day 11 part 1 ");
    println!("Start:");
    monkey_life.print_monkeys();

    for _h in 0..20 {
        for i in 0..monkey_life.monkeys.len() {
            let to_others = monkey_life.monkeys[i].do_turn(3);
            for (m, v) in to_others {
                monkey_life.monkeys[m as usize].add_item(v);
            }
        }
        println!("After round {} :",_h+1);
        monkey_life.print_monkeys();
    }
     let result:Vec<i64> = monkey_life.monkeys.iter().sorted_by_key(|m|-m.get_count()).map(|m|m.get_count()).take(2).collect_vec();

    println!("two largest {} * {} = {}",result[0],result[1],result[0]*result[1]);


    println!("Day 11 part 2 ");
    let mut monkey_life = MonkeyParser::new();
    parse_input_file("src/day11/input.txt", &mut monkey_life).unwrap();
    monkey_life.set_common_devisor();

    for _h in 0..10000 {
        for i in 0..monkey_life.monkeys.len() {
            let to_others = monkey_life.monkeys[i].do_turn(1);
            for (m, v) in to_others {
                monkey_life.monkeys[m as usize].add_item(v);
            }
        }

     }
    println!("After last round:");
    monkey_life.print_monkeys();
    let result:Vec<i64> = monkey_life.monkeys.iter().sorted_by_key(|m|-m.get_count()).map(|m|m.get_count()).take(2).collect_vec();

    println!("two largest {} * {} = {}",result[0],result[1],result[0] * result[1] );

    Ok(())
}

//fn parse_input_line(line:&str) -> usize{ line.len()}
