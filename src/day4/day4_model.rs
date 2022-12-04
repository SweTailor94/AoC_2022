// model types for Day4

use range_ext::intersect::Intersect;

pub struct AssignmentPair{
    area1_start: i32,
    area1_end:  i32,
    area2_start: i32,
    area2_end:  i32,
}


impl AssignmentPair {

    pub fn new(line: &str) -> AssignmentPair{
        let areas:Vec<(i32,i32)> = line.split(",").map(|pair| {
            let numbers:Vec<i32> = pair.split("-").map(|num| num.parse::<i32>().unwrap()).collect();
            (numbers[0],numbers[1])
        }).collect();

        AssignmentPair{
            area1_start: areas[0].0,
            area1_end: areas[0].1,
            area2_start: areas[1].0,
            area2_end:  areas[1].1
        }
    }

    pub fn is_one_contained(&self) -> bool {
        (self.area1_start <= self.area2_start && self.area1_end >= self.area2_end) ||
            (self.area2_start <= self.area1_start && self.area2_end >= self.area1_end)
    }

    pub fn is_overlapping(&self) -> bool{
        let r1 = self.area1_start .. self.area1_end + 1 ;
        let r2 = self.area2_start .. self.area2_end + 1;
        r1.intersect(&r2).is_any()
    }
}