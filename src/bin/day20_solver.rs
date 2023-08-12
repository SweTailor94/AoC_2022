use itertools::Itertools;
use advent_of_code_2022::day20::day20_model::Sequence;
use advent_of_code_2022::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut encrypted = Sequence::new();
    let _ = parse_input_file("src/day20/input.txt", &mut encrypted);
    //println!("Day 20 part 1 ");
    //println!("Encrypted file len {}. Is multiwrap {}", encrypted.length, encrypted.is_multi_wrap());
    //encrypted.mix_alt();
    //let mut sorted = encrypted.vals.clone();
    //sorted.sort_by_key(|n|n.current_index);
    //println!("{:?}", sorted);
    //println!("{:?}",sorted.iter().map(|n|n.val).collect_vec()) ;
    //println!("sum of coefficients {}", encrypted.find_sum());
    println!("Day 20 part 2 ");
    let decryption_key = 811589153 as i64;
    let mut encrypted2 = Sequence::new();
    let _ = parse_input_file("src/day20/input.txt", &mut encrypted2);
    encrypted2.apply_key(decryption_key);
    for i in 1..11 {
        encrypted2.mix_alt();
        println!("{} mix done.",i);
    }

    println!("sum of coefficients {}", encrypted2.find_sum());
      Ok(())
}


