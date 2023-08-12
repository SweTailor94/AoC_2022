// model types for Day21


use std::collections::HashMap;
use std::fmt::format;
use itertools::Itertools;
use crate::input::InputParser;

pub enum Monkey {
    Constant{ name:String, value: i64},
    Add{name:String, l_op:String, r_op:String},
    Sub{name:String, l_op:String, r_op:String},
    Mul{name:String, l_op:String, r_op:String},
    Div{name:String, l_op:String, r_op:String},
}

pub struct MathMonkeys {
    pub monkeys: HashMap<String,Monkey>,
    pub is_part2: bool,
    pub humn: i64,
}

impl MathMonkeys {
    pub fn new() -> MathMonkeys{
        MathMonkeys{
            monkeys: HashMap::new(),
            is_part2: false,
            humn: 0,
        }
    }

    pub fn get_value_from_monkey(& self,name:&String) -> i64{
        if self.is_part2 && name == "humn" {
            return self.humn;
        }
        let m = match self.monkeys.get(name) {
            None => { panic!("Missing monkey {} in herd.",name)}
            Some(m) => {
                m
            }
        };
        match m {
            Monkey::Constant { value, ..} => {*value}
            Monkey::Add { l_op,r_op, .. } => {
                self.get_value_from_monkey(l_op) + self.get_value_from_monkey(r_op)
            }
            Monkey::Sub {  l_op,r_op, .. } => {
                self.get_value_from_monkey(l_op) - self.get_value_from_monkey(r_op)
            }
            Monkey::Mul {  l_op,r_op, ..} => {
                self.get_value_from_monkey(l_op) * self.get_value_from_monkey(r_op)
            }
            Monkey::Div {  l_op,r_op, ..} => {
                self.get_value_from_monkey(l_op) / self.get_value_from_monkey(r_op)
            }
        }
    }
    pub fn print_expr(& self,name:&String) -> (Option<i64>,String){
        if self.is_part2 && name == "humn" {
            return (None,"x".to_string())
        }
        let m = match self.monkeys.get(name) {
            None => { panic!("Missing monkey {} in herd.",name)}
            Some(m) => {
                m
            }
        };
        match m {
            Monkey::Constant { value, ..} => {(Some(*value),"".to_string())}
            Monkey::Add { l_op,r_op, .. } => {
                match ( self.print_expr(l_op) , self.print_expr(r_op)) {
                    ((Some(lv),_),(Some(rv),_)) => { (Some(lv+rv),"".to_string()) }
                    ((Some(lv),_),(None,s) )=> { (None, format!("({}+{})",lv,s)) }
                    ((None,s),(Some(rv),_)) => { (None, format!("({}+{})",s,rv)) }
                    ((None,sl),(None,sr)) => { (None,format!("({}+{}",sl,sr)) }
                }
            }
            Monkey::Sub {  l_op,r_op, .. } => {
                match ( self.print_expr(l_op) , self.print_expr(r_op)) {
                    ((Some(lv),_),(Some(rv),_)) => { (Some(lv-rv),"".to_string()) }
                    ((Some(lv),_),(None,s) )=> { (None, format!("({}-{})",lv,s)) }
                    ((None,s),(Some(rv),_)) => { (None, format!("({}-{})",s,rv)) }
                    ((None,sl),(None,sr)) => { (None,format!("({}-{}",sl,sr)) }
                }
           }
            Monkey::Mul {  l_op,r_op, ..} => {
                match ( self.print_expr(l_op) , self.print_expr(r_op)) {
                    ((Some(lv),_),(Some(rv),_)) => { (Some(lv*rv),"".to_string()) }
                    ((Some(lv),_),(None,s) )=> { (None, format!("({}*{})",lv,s)) }
                    ((None,s),(Some(rv),_)) => { (None, format!("({}*{})",s,rv)) }
                    ((None,sl),(None,sr)) => { (None,format!("({}*{}",sl,sr)) }
                }
            }
            Monkey::Div {  l_op,r_op, ..} => {
                match ( self.print_expr(l_op) , self.print_expr(r_op)) {
                    ((Some(lv),_),(Some(rv),_)) => { (Some(lv/rv),"".to_string()) }
                    ((Some(lv),_),(None,s) )=> { (None, format!("({}/{})",lv,s)) }
                    ((None,s),(Some(rv),_)) => { (None, format!("({}/{})",s,rv)) }
                    ((None,sl),(None,sr)) => { (None,format!("({}/{}",sl,sr)) }
                }
           }
        }
    }
    pub fn simplify (&mut self,name:&String) {
        let mut changed = HashMap::new();
        let _ = self.simplify_from(name,&mut changed);
        println!("Changed {} to constants",changed.len());
        for (name, monkey) in changed.drain(){
            self.monkeys.remove(&name);
            self.monkeys.insert(name,monkey);
        }
    }

    fn simplify_from(& self, name:&String, mut changed: &mut HashMap<String, Monkey>) -> Option<i64> {
        if self.is_part2 && name == "humn" {
            return None;
        }
        let m = match self.monkeys.get(name) {
            None => { panic!("Missing monkey {} in herd.",name)}
            Some(m) => {
                m
            }
        };
        match m {
            Monkey::Constant { value, ..} => {Some(*value)}
            Monkey::Add { l_op,r_op, .. } => {
                match (self.simplify_from(l_op, &mut changed) , self.simplify_from(r_op, &mut changed)) {
                    (Some(l),Some(r)) => {
                        changed.insert( name.clone(), Monkey::Constant {name:name.clone(),value: l+r });
                        Some(l+r)
                    }
                    _ => {None}
                }
            }
            Monkey::Sub {  l_op,r_op, .. } => {
                match (self.simplify_from(l_op, &mut changed) , self.simplify_from(r_op, &mut changed)) {
                    (Some(l),Some(r)) => {
                        changed.insert( name.clone(),  Monkey::Constant {name:name.clone(),value: l-r });
                        Some(l-r)
                    }
                    _ => {None}
                }
            }
            Monkey::Mul {  l_op,r_op, ..} => {
                match (self.simplify_from(l_op, &mut changed) , self.simplify_from(r_op, &mut changed)) {
                    (Some(l),Some(r)) => {
                        changed.insert( name.clone(), Monkey::Constant {name:name.clone(),value: l*r });
                        Some(l*r)
                    }
                    _ => {None}
                }
            }
            Monkey::Div {  l_op,r_op, ..} => {
                match (self.simplify_from(l_op, &mut changed) , self.simplify_from(r_op, &mut changed)) {
                    (Some(l),Some(r)) => {
                        changed.insert( name.clone(),  Monkey::Constant {name:name.clone(),value: l/r });
                        Some(l/r)
                    }
                    _ => {None}
                }
            }
        }
    }
}

impl InputParser for MathMonkeys {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let m_op = line.split(":").collect_vec();
        let op = m_op[1].trim().split(" ").collect_vec();
        if op.len() == 1 {
            self.monkeys.insert(m_op[0].to_string(),
                                Monkey::Constant {
                                    name:m_op[0].to_string(),
                                    value: op[0].parse().unwrap()
                                } );
        } else if op.len() == 3 {
            match op[1] {
                "+" => self.monkeys.insert(m_op[0].to_string(),
                                           Monkey::Add {
                                               name:m_op[0].to_string(),
                                               l_op: op[0].to_string(),
                                               r_op: op[2].to_string(),
                                           } ),
                "-" => self.monkeys.insert(m_op[0].to_string(),
                                           Monkey::Sub {
                                               name:m_op[0].to_string(),
                                               l_op: op[0].to_string(),
                                               r_op: op[2].to_string(),
                                           } ),
                "*" => self.monkeys.insert(m_op[0].to_string(),
                                           Monkey::Mul {
                                               name:m_op[0].to_string(),
                                               l_op: op[0].to_string(),
                                               r_op: op[2].to_string(),
                                           } ),
                "/" => self.monkeys.insert(m_op[0].to_string(),
                                           Monkey::Div {
                                               name:m_op[0].to_string(),
                                               l_op: op[0].to_string(),
                                               r_op: op[2].to_string(),
                                           } ),
                s => panic!("Illegal op {}",s),
            };
        } else {
            panic!("Strange line {}",line);
        };

        Ok(())
    }
}