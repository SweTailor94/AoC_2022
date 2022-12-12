// model types for Day12

use std::collections::{HashSet, VecDeque};
use crate::input::InputParser;

pub struct ElfMap{
    pub heights: Vec<Vec<u8>>,
    pub start: (i32,i32), // (row, col) 0 .. size
    pub end: (i32,i32),  // (row,col)
}
impl ElfMap {
    pub fn path_finder(&self) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut next_try: VecDeque<((i32, i32), i32)> = VecDeque::new();
        next_try.push_back((self.start, 0));
        while next_try.len() > 0{
            let next = next_try.pop_front().unwrap();
            if visited.contains(&next.0) { continue;}
            if next.0 == self.end {
                //done
                return next.1;
            }

            for p in self.all_possible(&next.0){
                next_try.push_back((p,next.1+1));
            }
            visited.insert(next.0);
        }
        -1
    }
    fn all_possible(&self, pos:&(i32,i32)) -> Vec<(i32,i32)> {
        let mut result = Vec::new();
        let current_height = self.heights[pos.0 as usize][pos.1 as usize] as i16;
        // left
        let new_col = pos.1 -1;
        if new_col >= 0 && (self.heights[pos.0 as usize][new_col as usize] as i16 - current_height <= 1) {
            result.push((pos.0, new_col));
        }
        // right
        let new_col = 1 + pos.1;
        if new_col < self.heights[pos.0 as usize].len() as i32 && (self.heights[pos.0 as usize][new_col as usize] as i16 - current_height <= 1) {
            result.push((pos.0, new_col));
        }
        // up
        let new_row = pos.0 -1;
        if new_row >= 0 && (self.heights[new_row as usize][pos.1 as usize] as i16 - current_height <= 1) {
            result.push((new_row, pos.1));
        }
        // down
        let new_row2 = pos.0 + 1;
        if new_row2 < self.heights.len() as i32 && ( self.heights[new_row2 as usize][pos.1 as usize] as i16 - current_height) <= 1 {
            result.push((new_row2, pos.1));
        }

        result
    }
    pub fn path_finder_b(&self) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut next_try: VecDeque<((i32, i32), i32)> = VecDeque::new();
        next_try.push_back((self.end, 0));
        while next_try.len() > 0{
            let next = next_try.pop_front().unwrap();
            if visited.contains(&next.0) { continue;}
            if self.heights[next.0.0 as usize][next.0.1 as usize] == b'a'  {
                //done
                return next.1;
            }

            for p in self.all_possible_b(&next.0){
                next_try.push_back((p,next.1+1));
            }
            visited.insert(next.0);
        }
        -1
    }
    fn all_possible_b(&self, pos:&(i32,i32)) -> Vec<(i32,i32)> {
        let mut result = Vec::new();
        let current_height = self.heights[pos.0 as usize][pos.1 as usize] as i16;
        // left
        let new_col = pos.1 -1;
        if new_col >= 0 && (self.heights[pos.0 as usize][new_col as usize] as i16 - current_height >= -1) {
            result.push((pos.0, new_col));
        }
        // right
        let new_col = 1 + pos.1;
        if new_col < self.heights[pos.0 as usize].len() as i32 && (self.heights[pos.0 as usize][new_col as usize] as i16 - current_height >= -1) {
            result.push((pos.0, new_col));
        }
        // up
        let new_row = pos.0 -1;
        if new_row >= 0 && (self.heights[new_row as usize][pos.1 as usize] as i16 - current_height >= -1) {
            result.push((new_row, pos.1));
        }
        // down
        let new_row2 = pos.0 + 1;
        if new_row2 < self.heights.len() as i32 && ( self.heights[new_row2 as usize][pos.1 as usize] as i16 - current_height) >= -1 {
            result.push((new_row2, pos.1));
        }

        result
    }
}
impl InputParser for ElfMap {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let mut row =  line.as_bytes().to_vec();
        if let Some(col) = index_of(&row,b'S') {
            self.start = (self.heights.len() as i32, col);
            row[col as usize] = b'a';
        }
        if let Some(col) = index_of(&row,b'E') {
            self.end = (self.heights.len() as i32, col);
            row[col as usize] = b'z'; // we now know where this is, so set it to correct height
        }
        self.heights.push(row);
        Ok(())
    }
}

fn index_of(row:&Vec<u8>, v:u8)-> Option<i32> {
    for i in 0..row.len() {
        if row[i] == v {
            return Some(i as i32);
        }
    }
    None
}

