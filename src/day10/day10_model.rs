// model types for Day10

use std::ffi::c_char;
use itertools::chain;

pub enum Op {
    Nop,
    Addx(i32),
}

pub struct Computer {
    program: Vec<Op>,
    acc_x: i32,
    p_pointer: usize,
    cycles: i32,
    render_from_last_cycle: Option<char>,
}

impl Computer {
    pub fn new(program: Vec<Op>) -> Computer {
        Computer {
            program,
            acc_x: 1,
            p_pointer: 0,
            cycles: 0,
            render_from_last_cycle: None,
        }
    }

    pub fn run_until_cycle(&mut self, cycle: i32) -> i32 { // returns the "Signal strengh" Cycle * acc_x
        if self.cycles > cycle { panic!("Demanded cycle already passed!") };
        let mut tmp_cycles = self.cycles;
        let mut delta = 0;
        while (tmp_cycles < cycle) {
            // add delta from last cycle
            self.acc_x += delta;
            if self.p_pointer >= self.program.len() {
                panic!("Program ended")
            }
            match self.program[self.p_pointer] {
                Op::Nop => {
                    tmp_cycles += 1;
                    delta = 0;
                }
                Op::Addx(v) => {
                    tmp_cycles += 2;
                    delta = v;
                }
            }
            self.p_pointer += 1;
        }
        self.cycles = tmp_cycles;
        let signal_strength = self.acc_x * cycle;
        self.acc_x += delta;
        signal_strength
    }

    pub fn run_until_cycle2(&mut self, cycle: i32) -> Option<String> { // returns a rendered row
        if self.cycles > cycle { panic!("Demanded cycle already passed!") };
        let mut tmp_cycles = self.cycles;
        let mut delta = 0;
        let mut crt_row: String = String::new();
        if let Some(ch) = self.render_from_last_cycle {
            crt_row.push(ch);
        }
        while (tmp_cycles < cycle) {
            // add delta from last cycle
            self.acc_x += delta;
            if self.p_pointer >= self.program.len() {
                return None;
            }
            match self.program[self.p_pointer] {
                Op::Nop => {

                    if (self.acc_x - (tmp_cycles % cycle)).abs() < 2 {
                        crt_row.push('#');
                    }else{
                        crt_row.push(' ');
                    }
                    tmp_cycles += 1;
                    delta = 0;
                }
                Op::Addx(v) => {
                    for _i in 0..2 {

                        if (self.acc_x -(tmp_cycles % cycle)).abs() < 2 {
                            crt_row.push('#');
                        }else{
                            crt_row.push(' ');
                        }
                        tmp_cycles += 1;
                    }
                    delta = v;
                }
            }
            self.p_pointer += 1;
        }
        self.cycles = tmp_cycles % cycle;
        self.acc_x += delta;
        if crt_row.len() > cycle as usize{
            self.render_from_last_cycle = crt_row.pop();
        }
        else { self.render_from_last_cycle = None; }
        Some(crt_row)
    }
}

