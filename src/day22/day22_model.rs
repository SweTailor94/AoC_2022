use std::io::Cursor;
use crate::input::InputParser;

// model types for Day22

pub enum Move {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

enum ParseState {
    BoardParsing,
    MoveR,
    MoveL,
    MoveU,
    MoveD,
}
pub struct Board {
    pub rows : Vec<bool>,
    pub col_offset: Vec<i32>,
    pub moves: Vec<Move>,
    state: ParseState,
}
impl Board {
    pub fn new() -> Board{
        Board {
            rows: Vec::new(),
            col_offset: Vec::new(),
            moves: Vec::new(),
            state: ParseState::BoardParsing,
        }
    }

    pub fn walk(&mut self) ->(i32,i32,i32) { // (row,col,facing)
        let cur_row = 1;
        let cur_col = 1;

        (cur_row,cur_col,0)
    }
}

impl InputParser for Board {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        if line.is_empty() {
            self.state = ParseState::MoveR;
        } else {
            match self.state {
                ParseState::BoardParsing => {

                }
                ParseState::MoveR |
                ParseState::MoveL |
                ParseState::MoveU |
                ParseState::MoveD => {
                    build_moves( self,line);
                }
            }

        }
        Ok(())
    }
}

fn build_moves(board: &mut Board, moves: &String) {
    let mut stream : Cursor<&String> = Cursor::new(moves);
    // let mut steps:i32 = stream.read_i32().into();
}

fn turn(state:ParseState,turn_dir:char) -> ParseState {
    match (state,turn_dir) {
        (ParseState::MoveR,'R') => ParseState::MoveD,
        (ParseState::MoveR,'L') => ParseState::MoveU,
        (ParseState::MoveL,'R') => ParseState::MoveU,
        (ParseState::MoveL,'L') => ParseState::MoveD,
        (ParseState::MoveU,'R') => ParseState::MoveR,
        (ParseState::MoveU,'L') => ParseState::MoveL,
        (ParseState::MoveD,'R') => ParseState::MoveL,
        (ParseState::MoveD,'L') => ParseState::MoveR,
        _ => panic!("faulty input to turn()"),
    }
}