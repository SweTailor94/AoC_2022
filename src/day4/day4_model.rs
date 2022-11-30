// model types for Day4


use std::fmt::{Display, Formatter};
use anyhow::bail;
use crate::input::InputParser;

#[derive(Clone, Copy, Debug)]
pub enum Number {
    Unmarked(i32),
    Marked(i32),
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Unmarked(n) => write!(f, "[ ]{:02}", n),
            Number::Marked(n) => write!(f, "[x]{:02}", n),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bingo {
    pub board: [[Number; 5]; 5],
    pub got_bingo: bool,
}

impl Bingo {
    pub fn new() -> Bingo {
        Bingo {
            board: [[Number::Unmarked(0); 5]; 5],
            got_bingo: false,
        }
    }

    pub fn print(&self) {
        for row in 0..5 {
            println!("{} {} {} {} {}",
                     self.board[row][0],
                     self.board[row][1],
                     self.board[row][2],
                     self.board[row][3],
                     self.board[row][4], );
        }
    }

    /// Returns true if the board has bingo else false
    pub fn mark_number(&mut self,number: i32) -> Option<i32>{
        for row in 0..5 {
            for col in 0..5 {
                match self.board[row][col] {
                    Number::Unmarked(n)|Number::Marked(n) =>{
                        if n == number  { self.board[row][col] = Number::Marked(n); }
                    }
                }
            }
        }
        if let Some(sum) = self._has_bingo() {
            return Some(sum * number);
        }
        None
    }

    fn _has_bingo(&mut self) -> Option<i32>{
        if self.got_bingo {
            return None;
        }
        let mut bingo_col =  [true;5];
        for row in 0..5 {
            let mut bingo_this_row = true;
            for col in 0..5 {
                let  is_marked = match self.board[row][col] {
                    Number::Unmarked(_) => false,
                    Number::Marked(_) => true,
                };
                bingo_col[col] = bingo_col[col] && is_marked;
                bingo_this_row = bingo_this_row && is_marked;
            }
            if bingo_this_row {
                self.got_bingo = true;
                return Some(self.get_score());
            }
        }
        for i in 0..5 {
            if bingo_col[i] {
                self.got_bingo = true;
                return Some(self.get_score());
            }
        }
        None
    }

    fn get_score(&self) -> i32 {
        let mut score = 0;
        for r in 0..5{
            for c in 0..5{
                match self.board[r][c] {
                    Number::Unmarked(n) => {score += n;}
                    Number::Marked(_) => {}
                }
            }
        }
        score
    }
}

pub enum ParseState {
    Firstline,
    BingoBoard(usize, Bingo),
    Space,

}

pub struct InputData {
    pub draw: Vec<i32>,
    pub boards: Vec<Bingo>,
    state: ParseState,
}

impl InputData {
    pub fn new() -> InputData {
        InputData {
            draw: Vec::new(),
            boards: Vec::new(),
            state: ParseState::Firstline,
        }
    }
}

impl InputParser for InputData {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        match &self.state {
            ParseState::Firstline => {
                self.draw = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
                self.state = ParseState::Space;
            }

            ParseState::Space => {
                if line.is_empty() {
                    self.state = ParseState::BingoBoard(0, Bingo::new());
                } else { bail!("Input Error, should be an empty line here. {}",line) }
            }
            ParseState::BingoBoard(mut row, mut board) => {
                let parts: Vec::<&str> = line.split_whitespace().collect();
                let cols: Vec::<i32> = parts.iter().map(|s| s.parse::<i32>().unwrap_or(-1)).collect();
                if cols.len() != 5 {
                    bail!("Line should contain 5 numbers. {}", line)
                }
                for i in 0..5 {
                    board.board[row][i] = Number::Unmarked(cols[i]);
                }
                row += 1;
                if row >= 5 {
                    self.boards.push(board);
                    self.state = ParseState::Space;
                } else {
                    self.state = ParseState::BingoBoard(row, board);
                }
            }
        };
        Ok(())
    }
}

