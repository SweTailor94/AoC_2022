use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day1/input.txt", parse_input_line);
    println!("Day 1 part 1 ");
    let mut calorie_sum : Vec<i32> = Vec::new();
    let mut temp_sum = 0;
    for c in input{
        if c.is_empty() {
            calorie_sum.push(temp_sum);
            temp_sum = 0;
        }
        else
        {
            temp_sum += c.parse::<i32>().unwrap();
        }
    }
    calorie_sum.push(temp_sum);
    calorie_sum.sort_by(|a, b| b.cmp(a));
    let max = calorie_sum[0];
    println!("Elf with most calories {} ",max);


    println!("\nDay 1 part 2 ");
    for c in &calorie_sum{
        println!("{}",*c);
    }
    println!("Top three elfs carrY {} calories",calorie_sum[0]+calorie_sum[1]+calorie_sum[2]);
      Ok(())
}

fn parse_input_line(line:&str) -> String{
    line.to_string()
}
