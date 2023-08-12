// model types for Day15

use std::collections::HashMap;
use std::ops::Range;


pub fn try_concat_range(r1: &Range<i64>, r2: &Range<i64>) -> Option<Range<i64>> {
    if touches(r1, r2.start) {
        if r2.end > r1.end {
            Some(r1.start..r2.end)
        } else {
            Some(r1.start..r1.end)
        }
    } else if touches(r2, r1.start) {
        if r1.end > r2.end {
            Some(r2.start..r1.end)
        } else {
            Some(r2.start..r2.end)
        }
    } else {
        None
    }
}

pub fn touches(r1: &Range<i64>, v: i64) -> bool {
    v >= r1.start && v <= r1.end
}

#[derive(Debug)]
pub struct Ranges
{
    pub covered: Vec<Range<i64>>,
}

impl Ranges {
    pub(crate) fn is_full(&self) -> bool {
        self.covered.len() == 1 &&
            self.covered[0].start <= 0 &&
            self.covered[0].end >= 4000001
    }
}

impl Ranges {
    pub fn add_range_to_ranges(&mut self, p: bool, r: Range<i64>) {
        if r.is_empty() { panic!("this range is faulty {:?}", r); }
        if p { println!("adding: {:?}",r);}
        self.covered.push(r);
        self.covered.sort_by_key(|r| r.start);
        if p {println!("{:?}",self.covered);}
        if self.covered.len() > 1 {
            let mut i = 0;
            while i < self.covered.len() - 1 {
                match try_concat_range(&self.covered[i], &self.covered[i + 1]) {
                    None => {
                        if p {println!("i = {} no match.",i); }
                        i += 1;
                    }
                    Some(new_r) => {
                        self.covered.remove(i + 1);
                        self.covered[i] = new_r;
                    }
                }
                if p {println!("{:?}",self.covered);}
            }
        }
    }
}

pub struct Sensor {
    pub x: i64,
    pub y: i64,
    pub b_x: i64,
    pub b_y: i64,
}

impl Sensor {
    pub fn covered_pos_on_y(&self, y: i64) -> Vec<(i64, i64)> {
        let mut covered = Vec::new();
        let my_dist = manhattan_dist((self.x, self.y), (self.b_x, self.b_y));
        let n = (self.y - y).abs();
        if n <= my_dist {
            let extra = my_dist - n;
            for i in self.x - extra..self.x + extra {
                covered.push((i, y));
            }
        }
        covered
    }

    pub fn add_covered_in_line_ranges(&self, lines: &mut HashMap<i64, Ranges>) {
        let my_dist = manhattan_dist((self.x, self.y), (self.b_x, self.b_y));
        if let Some(line) = lines.get_mut(&self.y) {
            let p = 1605601 == self.y;
            line.add_range_to_ranges(p, (self.x - my_dist).max(0)..(self.x + my_dist + 1).min(4000001));
            if line.is_full() {
                lines.remove(&self.y);
            }
        }
        for i in 1..my_dist + 1
        {
            let steps = my_dist - i;

            if let Some(line) = lines.get_mut(&(self.y - i)) {
                let p = 1605601 == self.y - i;
                line.add_range_to_ranges(p, (self.x - steps).max(0)..(self.x + steps + 1).min(4000001));
                if line.is_full() {
                    lines.remove(&(self.y - i));
                }
            }
            if let Some(line) = lines.get_mut(&(self.y + i))
            {
                let p = 1605601 == self.y + i;
                line.add_range_to_ranges(p, (self.x - steps).max(0)..(self.x + steps + 1).min(4000001));
                if line.is_full() {
                    lines.remove(&(self.y + i));
                }
            }
        }
    }
}

pub fn manhattan_dist(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::day15::day15_model::{Ranges, try_concat_range};

    #[test]
    pub fn ranges_test() {
        // let mut my: HashMap<i64,Ranges> = HashMap::new();
        let mut my = Ranges { covered: Vec::new() };
        let r = 5..8;
        my.add_range_to_ranges(false ,r);
        let r = 10..90;
        my.add_range_to_ranges(false ,r);
        let r = 6..10;
        my.add_range_to_ranges(false ,r);
        assert_eq!(my.covered.len(), 1);
    }

    #[test]
    pub fn ranges_test2() {
        // let mut my: HashMap<i64,Ranges> = HashMap::new();
        let mut my = Ranges { covered: Vec::new() };
        let r = 10..90;
        my.add_range_to_ranges(false ,r);
        let r = 5..8;
        my.add_range_to_ranges(false ,r);
        let r = 6..10;
        my.add_range_to_ranges(false ,r);
        assert_eq!(my.covered.len(), 1);
    }

    #[test]
    pub fn ranges_test3() {
        // let mut my: HashMap<i64,Ranges> = HashMap::new();
        let mut my = Ranges { covered: Vec::new() };
        let r = 10..90;
        my.add_range_to_ranges(false ,r);
        let r = 5..8;
        my.add_range_to_ranges(false ,r);
        let r = 90..200;
        my.add_range_to_ranges(false ,r);
        let r = 6..10;
        my.add_range_to_ranges(false ,r);
        assert_eq!(my.covered.len(), 1);
    }

    #[test]
    pub fn test_try_concat_range1()
    {
        let r1 = 5..8;
        let r2 = 7..12;
        let r3 = try_concat_range(&r1, &r2);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r1.start);
                assert_eq!(r.end, r2.end);
            }
        }
        let r3 = try_concat_range(&r2, &r1);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r1.start);
                assert_eq!(r.end, r2.end);
            }
        }
    }

    #[test]
    pub fn test_try_concat_range2()
    {
        let r1 = 5..8;
        let r2 = 8..12;
        let r3 = try_concat_range(&r1, &r2);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r1.start);
                assert_eq!(r.end, r2.end);
            }
        }
        let r3 = try_concat_range(&r2, &r1);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r1.start);
                assert_eq!(r.end, r2.end);
            }
        }
    }

    #[test]
    pub fn test_try_concat_range3()
    {
        let r1 = 5..8;
        let r2 = -7..12;
        let r3 = try_concat_range(&r1, &r2);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r2.start);
                assert_eq!(r.end, r2.end);
            }
        }
        let r3 = try_concat_range(&r2, &r1);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r2.start);
                assert_eq!(r.end, r2.end);
            }
        }
    }

    #[test]
    pub fn test_try_concat_range4()
    {
        let r1 = 5..8;
        let r2 = -7..7;
        let r3 = try_concat_range(&r1, &r2);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r2.start);
                assert_eq!(r.end, r1.end);
            }
        }
        let r3 = try_concat_range(&r2, &r1);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r2.start);
                assert_eq!(r.end, r1.end);
            }
        }
    }

    #[test]
    pub fn test_try_concat_range5()
    {
        let r1 = 5..8;
        let r2 = 9..12;
        let r3 = try_concat_range(&r1, &r2);
        assert_eq!(r3, None);
        let r3 = try_concat_range(&r2, &r1);
        assert_eq!(r3, None);
    }

    #[test]
    pub fn test_big() {
        let r1 = -1319852..3040676;
        let r2 = 2759735..4288102;
        let r3 = try_concat_range(&r2, &r1);
        match r3 {
            None => assert!(false, "Should concat!"),
            Some(r) => {
                assert_eq!(r.start, r1.start);
                assert_eq!(r.end, r2.end);
            }
        }
    }
}