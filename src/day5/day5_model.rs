// model types for Day5

use std::collections::VecDeque;
use std::str::from_utf8;
use std::vec;
use crate::input::InputParser;

pub struct Stacks{
    stacks: Vec<VecDeque<u8>>
}

impl Stacks{
    pub fn new() -> Stacks{
        Stacks{
            stacks: Vec::new(),
        }
    }

    pub fn add_to_stack(&mut self, index: usize, letter: u8 ){
        while self.stacks.len()  < (index + 1)   {
            self.stacks.push(VecDeque::new());
        }
        self.stacks[index].push_back(letter);
    }

    pub fn move_crates(&mut self, count:i32, from:usize, to: usize){
        for i in 0..count {
            let f = self.stacks[from].pop_front().unwrap();
            self.stacks[to].push_front(f);
        }
    }

    pub fn move_crates_9001(&mut self, count:i32, from:usize, to: usize){
        let mut tmp : VecDeque<u8> = VecDeque::new();
        for _i in 0..count {
            tmp.push_front(self.stacks[from].pop_front().unwrap());
        }
        for _i in 0..count {
            self.stacks[to].push_front(tmp.pop_front().unwrap());
        }
    }

    pub fn get_top_string(&self) -> String {
        let mut string: Vec<u8> = vec!();
        for stack in &self.stacks {
            string.push(*stack.front().unwrap());
        }
        from_utf8(&string).unwrap().to_string()
    }
}

pub struct CraneMove{
    pub count:i32,
    pub from:usize,
    pub to:usize
}

pub struct Inputs{
    pub my_stacks: Stacks,
    pub moves: Vec<CraneMove>,
    state: ParseState,
}


impl Inputs{
    pub fn new() -> Inputs{
        Inputs{
            my_stacks: Stacks::new(),
            moves: Vec::new(),
            state: ParseState::Stacks,
        }
    }
}
enum ParseState{
    Stacks,
    Moves
}
impl InputParser for Inputs {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        match self.state{
            ParseState::Stacks => {
                let parse_line = line.clone().into_bytes();
                if parse_line.is_empty() {
                    self.state = ParseState::Moves;
                } else {
                    for i in (1..parse_line.len()).step_by(4) {
                        if parse_line[i] != b' ' {
                            self.my_stacks.add_to_stack((i-1)/4,parse_line[i] as u8);
                        }
                    }
                }
            }
            ParseState::Moves => {
                let tokens: Vec<&str> = line.split(" ").collect();
                self.moves.push(CraneMove{
                    count: tokens[1].parse::<i32>().unwrap(),
                    from: tokens[3].parse::<usize>().unwrap() -1,
                    to: tokens[5].parse::<usize>().unwrap()-1,
                })
            }
        }

        Ok(())
    }
}