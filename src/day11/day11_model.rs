// model types for Day11

use std::collections::VecDeque;
use std::ffi::c_int;
use std::ops::{Add, Div, Mul};
use std::vec;
use itertools::Itertools;
use crate::input::InputParser;


#[derive(Debug)]
pub enum Op {
    Nop,
    Add(i32),
    //Sub(i128),
    Mul(i32),
    // div(i128),
    Square,
}
#[derive(Debug)]
pub struct Monkey{
    items: VecDeque<u64>,
    operation: Op,
    divisible: u64,
    true_monkey: i32,
    false_monkey: i32,
    inspection_count:i64,
    common_divisor:u64,
}
impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            items: VecDeque::new(),
            operation: Op::Nop,
            divisible: 0,
            true_monkey: -1,
            false_monkey: -1,
            inspection_count: 0,
            common_divisor: 1,
        }
    }

    pub fn do_turn(&mut self, divider:i32) -> Vec<(i32,u64)> { // (monkey, value)
        let mut to_other_monkeys: Vec<(i32,u64)> = Vec::new();
        while let Some(item) = self.items.pop_front() {
            self.inspection_count += 1;
            let new_value = self.do_op(item,divider);
            to_other_monkeys.push((self.to_which_monkey(&new_value),new_value));
        }
        to_other_monkeys
    }

    fn to_which_monkey(&self, v:&u64) -> i32 {
         if v >= &self.divisible && v % &self.divisible == 0 {
             self.true_monkey
         }else{
             self.false_monkey
         }
    }

    fn do_op(&self, value:u64, divider:i32) -> u64 {
        let newval = match self.operation {
            Op::Nop => { panic!("Nop can't be evaluated.")}
            Op::Add(arg) => (value + arg as u64) / divider as u64,
            Op::Mul(arg) => value *  arg as u64 / divider as u64,
            Op::Square => value * value / divider as u64,
        };
        newval % self.common_divisor
    }

    pub fn add_item(&mut self, item:u64){
        self.items.push_back(item);
    }

    pub fn get_count(& self) -> i64 {
        self.inspection_count
    }
}
pub struct MonkeyParser {
    pub monkeys: Vec<Monkey>,
    current_monkey: i32,
}

impl MonkeyParser {
    pub fn new() -> MonkeyParser {
       MonkeyParser {
           monkeys: Vec::new(),
           current_monkey: -1,
       }
    }

    pub fn print_monkeys(&self){
        for i in 0..self.monkeys.len() {
            println!("Monkey{}: {:?}",i,self.monkeys[i]);
        }
    }

    pub fn set_common_devisor(&mut self) {
        let mut cd = 1;
        for m in self.monkeys.iter(){
            if cd % m.divisible != 0 {
                cd = cd * m.divisible;
            }
        }
        for m in self.monkeys.iter_mut(){
            m.common_divisor = cd;
        }
    }

}

impl InputParser for MonkeyParser{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let parts = line.trim().split(' ').collect_vec();
        match parts[0] {
            "Monkey" => { // Creta new monkey
                self.monkeys.push(Monkey::new());
                self.current_monkey +=1;
                if self.current_monkey != parts[1][..(parts[1].len()-1)].parse().unwrap() {
                    panic!("Parsing wrong monkey ?");
                }
            }
            "Starting" => {
                for v in parts.iter().skip(2) {
                    let mut value:u64;
                    if v.chars().last().unwrap() == ',' {
                        value = v[..(v.len()-1)].parse().unwrap();
                    }else {
                        value = v.parse().unwrap();
                    }
                    self.monkeys[self.current_monkey as usize].items.push_back(value);
                }
            }
            "Operation:" => {
                let op = if parts[5] == "old" {
                    Op::Square
                } else {
                    match parts[4] {
                        "*" => Op::Mul(parts[5].parse().unwrap()),
                        "+" => Op::Add(parts[5].parse().unwrap()),
                        _ => Op::Nop,
                    }
                };
                self.monkeys[self.current_monkey as usize].operation = op;
            }
            "Test:" => {
                let divisable = parts[3].parse().unwrap();
                self.monkeys[self.current_monkey as usize].divisible = divisable;
            }
            "If" =>{
                match parts[1] {
                    "true:" => self.monkeys[self.current_monkey as usize].true_monkey = parts[5].parse().unwrap(),
                    "false:" => self.monkeys[self.current_monkey as usize].false_monkey = parts[5].parse().unwrap(),
                    _ => panic!("Error in monkey test bool? {}",parts[1]),
                }
            }
            "" => {},
            _ => panic!("?? {}",line),
        }
        Ok(())
    }
}