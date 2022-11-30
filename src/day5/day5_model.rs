// model types for Day5
use anyhow::bail;

pub struct Line {
    pub x1:i32,
    pub y1:i32,
    pub x2:i32,
    pub y2 :i32,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    pub fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    pub fn is_hor_or_vert(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    pub fn new() -> Line{
        Line{x1: 0, y1: 0, x2: 0, y2: 0}
    }

    pub fn from_string( text_line:&String ) -> anyhow::Result<Line> {
        let points: Vec::<&str> = text_line.split("->").map(|s|s.trim()).collect();
        let p1: Vec::<&str> = points[0].split(",").collect();
        let p2: Vec::<&str> = points[1].split(",").collect();

        Ok(Line{
                x1: p1[0].parse::<i32>().unwrap(),
                y1: p1[1].parse::<i32>().unwrap(),
                x2: p2[0].parse::<i32>().unwrap(),
                y2: p2[1].parse::<i32>().unwrap(),
            }
        )
    }
}