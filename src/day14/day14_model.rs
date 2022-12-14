// model types for Day14

use std::collections::HashSet;
use itertools::Itertools;
use crate::input::InputParser;

pub struct Cave{
    pub world: HashSet<(i32,i32)>, // (x,y) of rock or sand
}



impl Cave {
    pub fn draw_line(&mut self, start:(i32,i32),end:(i32,i32)){
        if start.0 == end.0 { //vertical
            for y in start.1.min(end.1)..=start.1.max(end.1){
                self.world.insert((start.0,y));
            }
        } else if start.1 == end.1 { // Horizontal
            for x in start.0.min(end.0)..=start.0.max(end.0){
                self.world.insert((x,start.1));
            }
        } else { panic!("There shall be only vertical or horizontal lines!");}
    }

    pub fn sand_part1(&mut self) -> i32{
        let max_y = *self.world.iter().map(|(x,y)|y).max().unwrap();
        let mut sand_count = 0;
        loop{
            if self.run_one_unit(max_y) {
                sand_count += 1;
            } else {
                break;
            }
        }
        sand_count
    }

    pub fn sand_part2(&mut self) -> i32{
        let max_y = *self.world.iter().map(|(x,y)|y).max().unwrap();
        let mut sand_count = 0;
        loop{
            if self.run_one_unit_part2(max_y) {
                sand_count += 1;
            } else {
                break;
            }
        }
        sand_count
    }

    pub fn run_one_unit(&mut self, start_of_abyss:i32) -> bool {
        let mut x = 500;
        let mut y = 0 ;
        loop{
            if y >= start_of_abyss { return false;} // Sand is falling int o the abyss!
            if !self.world.contains(&(x,y+1) ){ // try strait down
                y += 1;
            } else if !self.world.contains(&(x-1,y+1)) {
                x -= 1;
                y += 1;
            } else if ! self.world.contains(&(x+1,y+1)) {
                x +=1;
                y +=1;
            } else { // Sand unit has stopped
                self.world.insert((x,y));
                return true;
            }
        }
    }


    pub fn run_one_unit_part2(&mut self, start_of_abyss:i32) -> bool {
        let mut x = 500;
        let mut y = 0 ;
        let floor = start_of_abyss+2;
        if self.world.contains(&(x,y)) {return false;} // No more sand can fall.
        loop{
            if y+1 < floor && !self.world.contains(&(x,y+1) ){ // try strait down
                y += 1;
            } else if y+1 < floor && !self.world.contains(&(x-1,y+1)) {
                x -= 1;
                y += 1;
            } else if y+1 < floor && !self.world.contains(&(x+1,y+1)) {
                x +=1;
                y +=1;
            } else { // Sand unit has stopped
                self.world.insert((x,y));
                return true;
            }
        }
    }
}

impl InputParser for Cave {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let corners = line.split("->").collect_vec();
        for c in 0..corners.len()-1{
            self.draw_line(str_to_xy(corners[c]), str_to_xy(corners[c+1]));
        }
        Ok(())
    }
}

fn str_to_xy(coord: &str) -> (i32,i32){
    let x_and_y = coord.trim().split(',').collect_vec();
    if x_and_y.len() != 2 { panic!("Strange coordinate {}",coord);}
    (x_and_y[0].parse().unwrap(),x_and_y[1].parse().unwrap())
}