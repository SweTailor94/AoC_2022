extern crate core;


use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::BufReader;
use itertools::Itertools;
use advent_of_code_2022::input::get_vector_from_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut input_data = get_vector_from_file("src/day13/input.txt", parse_input_line);
    let input = input_data.iter().filter(|s| !s.is_empty()).collect_vec();
    println!("Day 13 part 1 ");
    let s = get_ok_index(&input).iter().sum::<i32>();
    println!("Sum of index {}",s);


    println!("\nDay 13 part 2 ");
    input_data.push("[[2]]".to_string());
    input_data.push("[[6]]".to_string());
    let mut input = input_data.iter().filter(|s| !s.is_empty()).collect_vec();
    input.sort_by( |a,b| signal_comparer(a,b) );

    let mut index_2 = 0;
    let mut index_6= 0;
    for i in 0..input.len() {
        if input[i].to_string() == "[[2]]" {
            index_2 = i+1;
        }
        if input[i].to_string() == "[[6]]" {
            index_6 = i+1;
        }
        if index_2 > 0 && index_6 > 0 { break;}
    }

    println!("Decoder key {} * {} = {}",index_2,index_6, index_2*index_6);

    Ok(())
}

fn get_ok_index(inp: &Vec<&String>) -> Vec<i32> {
    let mut result = Vec::new();
    if inp.len() % 2 != 0{panic!("input must be even to form pairs")};
    let pairs = inp.len()/2;
    for i in 0..pairs{
        //println!("--------- Pair {} ------",i+1 );
        //println!("{:?} - {:?}",&inp[i * 2], &inp[i * 2 + 1]);
        if is_signal_ok(&inp[i * 2], &inp[i * 2 + 1]) {
            result.push(i as i32 + 1 ); // +1 index start at 1
        }
    }
    //println!("{:?}",result);
    result
}
#[derive(Debug,Clone)]
enum ElfList{
    List (Vec<ElfList>),
    Value(u8),
}

fn parse_to_elflist(line:&String) -> ElfList
{
    let mut l: VecDeque<u8> =  VecDeque::new( );
    l.extend(line.clone().as_bytes());
    let mut stack_of_elflist : VecDeque<ElfList> = VecDeque::new();
    loop{
        match l.pop_front() {
            None => break,
            Some(b'[') => {stack_of_elflist.push_back(
                ElfList::List(Vec::new())
            );}
            Some(b']') =>{
                let closed = stack_of_elflist.pop_back().unwrap();
                if stack_of_elflist.is_empty() {
                        return closed;
                }else{
                match   stack_of_elflist.back_mut() {
                    Some(ElfList::List(list) )=> list.push(closed),
                    _ => panic!("there should be a list here.")
                };
                }
            }
            Some(b',') => {} // no action go to next
            Some(v) => {
                let mut value: u8 = v - b'0';
                loop {
                    match l.pop_front() {
                        Some(b',') => {
                            match   stack_of_elflist.back_mut() {
                                Some(ElfList::List(list) )=> list.push(ElfList::Value(value)),
                                _ => panic!("there should be a list here.")
                            };
                            break}
                        Some(b'[') => panic!("Something wrong at {} from end in {} ",l.len(),line ),
                        Some(b']') => {
                            l.push_front(b']');
                            match   stack_of_elflist.back_mut() {
                                Some(ElfList::List(list) )=> list.push(ElfList::Value(value)),
                                _ => panic!("there should be a list here.")
                            };
                            break;
                        }
                        Some(dig) => {
                            value = value*10 + (dig-b'0');
                        }
                        None => break,
                    }
                }
            }
        }
    }

    match stack_of_elflist.pop_back() {
        None => { panic!("What??")}
        Some(result) => result,
    }
}

fn signal_comparer(left: &String, right  : &String) -> Ordering{
    let mut l  =  parse_to_elflist(left);
    let mut r =  parse_to_elflist(right);
    let x = is_correct(&l,&r);
    if x == 0 {
        return Ordering::Equal;
    }
    if x < 0 {
        return Ordering::Less;
    }
    Ordering::Greater
}

fn is_signal_ok(left: &String, right  : &String) -> bool {
    let mut l  =  parse_to_elflist(left);
    let mut r =  parse_to_elflist(right);
    let x = is_correct(&l,&r);
    if x == 0 {
        return true;
    }
    x < 0
}

fn is_correct(left: &ElfList, right: &ElfList) -> i32 {
    match (left, right) {
        (ElfList::Value(l), ElfList::Value(r)) => {
            return *l as i32 - *r as i32;
        }
        (ElfList::List(_), ElfList::Value(_)) => {
            let inner = vec![right.clone()];
            return is_correct(left, &ElfList::List(inner));
        }
        (ElfList::Value(_), ElfList::List(_)) => {
            let inner = vec![left.clone()];
            return is_correct(&ElfList::List(inner), right);
        }
        (ElfList::List(l_vec), ElfList::List(r_vec)) => {
            //println!("Comparing lists {:?} and {:?}", l_vec,r_vec);
            let mut i = 0;
            loop {
                if i >= l_vec.len() && i >= r_vec.len() {
                    //println!("lists equal");
                    return 0; }
                if i >= l_vec.len() {
                    //println!("left list smaller");
                    return -1; }
                if i >= r_vec.len() {
                    //println!("right list smaller");
                    return 1; }
                let tmp = is_correct(&l_vec[i], &r_vec[i]);
                if tmp != 0 { return tmp; }
                i += 1;
            }// loop
        }
    }
}

fn parse_input_line(line:&str) -> String{
    line.to_string()
}
