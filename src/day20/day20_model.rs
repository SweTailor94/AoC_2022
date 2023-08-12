// model types for Day20

use itertools::Itertools;
use crate::input::InputParser;

#[derive(Debug, Clone)]
pub struct Number {
    pub val: i64,
    pub original_index: i64,
    pub current_index: i64,
}

pub struct Sequence {
    pub vals: Vec<Number>,
    pub length: i64,
}

impl Sequence {
    pub fn find_sum(&self) -> i64 {
        if let Some(zero) = self.vals.iter().find(|n| n.val == 0) {
            let index1000 = (zero.current_index + 1000) % self.length;
            let index2000 = (zero.current_index + 2000) % self.length;
            let index3000 = (zero.current_index + 3000) % self.length;

            self.vals.iter().fold(0, |sum, v|
                if v.current_index == index1000 ||
                    v.current_index == index2000 ||
                    v.current_index == index3000 {
                    sum + v.val
                } else {
                    sum
                })
        } else {
            panic!("Something wrong!");
        }
    }
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence {
            vals: Vec::new(),
            length: 0,
        }
    }

    pub fn apply_key(&mut self, key:i64)
    {
        for num in self.vals.iter_mut(){
            num.val *= key;
        };
    }

    pub fn is_multi_wrap(&self) -> bool {
        let max = self.vals.len() as i64 * 2;
        self.vals.iter().any(|v| v.val.abs() > max)
    }

    pub fn mix(&mut self) {
        let length = self.vals.len();
        for i in 0..length {
            println!("step {}", i);
            //self.print_sorted();
            if self.vals[i].val == 0 {
                continue;
            }
            let tmp = self.vals[i].current_index;
            let new_index = tmp + self.vals[i].val;

            if self.vals[i].val > 0 { // Moving up
                if new_index >= length as i64 {
                    let start = tmp + 1;
                    let end = new_index - length as i64;
                    for num in self.vals.iter_mut() {
                        if num.current_index >= start || num.current_index <= end {
                            num.current_index -= 1;
                            if num.current_index < 0 {
                                num.current_index += length as i64;
                            }
                        }
                    }
                    self.vals[i].current_index = end;
                } else { // no wrap
                    let start = tmp + 1;
                    let end = new_index;
                    for num in self.vals.iter_mut() {
                        if num.current_index >= start && num.current_index <= end {
                            num.current_index -= 1;
                            if num.current_index < 0 {
                                num.current_index += length as i64;
                            }
                        }
                    }
                }
                self.vals[i].current_index = new_index;
            } else { // Moving down
                if new_index < 0 as i64 {
                    let start = new_index + length as i64;
                    let end = tmp;
                    for num in self.vals.iter_mut() {
                        if num.current_index >= start || num.current_index < end {
                            num.current_index += 1;
                            if num.current_index >= length as i64 {
                                num.current_index -= length as i64;
                            }
                        }
                    }
                    self.vals[i].current_index = start;
                } else { // no wrap
                    let start = new_index;
                    let end = tmp;
                    for num in self.vals.iter_mut() {
                        if num.current_index >= start && num.current_index < end {
                            num.current_index += 1;
                            if num.current_index >= length as i64 {
                                num.current_index -= length as i64;
                            }
                        }
                    }
                    self.vals[i].current_index = new_index;
                }
            }
            self.check_indexes();
        }
    }
    pub fn mix_alt(&mut self) {
        //self.print_sorted();
        let length = self.vals.len();
        for i in 0..length {
            //println!("step {}", i);

            if self.vals[i].val == 0 {
                continue;
            }
            let tmp = self.vals[i].current_index;
            let mut new_index = (tmp + self.vals[i].val) % (length as i64 -1); // TBD find why -1 here
            if new_index < 0 {
                new_index = new_index + length as i64 -1;
            };

            if new_index > tmp {

                    let start = tmp + 1;
                    let end = new_index ;
                    for num in self.vals.iter_mut() {
                        if num.current_index >= start && num.current_index <= end {
                            num.current_index -= 1;
                            if num.current_index < 0 {
                                num.current_index += length as i64;
                            }
                        }
                    }
                    self.vals[i].current_index = end;

            } else if new_index < tmp {

                    let start = new_index;
                    let end = tmp;
                    for num in self.vals.iter_mut() {
                        if num.current_index >= start && num.current_index < end {
                            num.current_index += 1;
                            if num.current_index >= length as i64 {
                                num.current_index -= length as i64;
                            }
                        }
                    }
                    self.vals[i].current_index = start;
            } // new_index == tmp -> do nothing.
            //self.print_sorted();
            //self.check_indexes();
        }
    }
    fn print_sorted(&self) {
        let mut sorted = self.vals.clone();
        sorted.sort_by_key(|v| v.current_index);
        println!("{:?}", sorted.iter().map(|v| (v.val, v.current_index)).collect_vec());
    }

    fn check_indexes(&self) {
        let mut sorted = self.vals.clone();
        sorted.sort_by_key(|v| v.current_index);
        for i in 0..sorted.len() {
            if i as i64 != sorted[i].current_index {
                panic!("index missmatch");
            }
        }
    }
}

impl InputParser for Sequence {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        self.vals.push(Number {
            val: line.parse().unwrap(),
            original_index: self.length,
            current_index: self.length,
        });
        self.length += 1;
        Ok(())
    }
}


#[cfg(test)]
mod test
{
    use crate::day20::day20_model::Sequence;
    use crate::input::parse_input_string;

    #[test]
    pub fn test1(){
        let test_input = "5\n1\n0\n-1".to_string();

        let mut test_sequence =  Sequence::new();
        let _ = parse_input_string(&test_input, &mut test_sequence);
        test_sequence.print_sorted();
        test_sequence.mix_alt();
        test_sequence.print_sorted();

    }
}
