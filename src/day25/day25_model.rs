// model types for Day25


use std::ops::Add;
use itertools::diff_with;

pub struct Snafu {
    pub dec: i64,
    pub rep: Vec<i8>,
}

impl Snafu {
    pub fn new(number: &str) -> Snafu {
        let digits = number.as_bytes();
        let mut rep: Vec<i8> = vec![0; digits.len()];
        let mut val: i64 = 0;
        let mut place: i64 = 1;
        let last = digits.len() - 1;
        for i in 0..digits.len() {
            match digits[last - i] {
                b'2' => {
                    val += place * 2;
                    rep[last - i] = 2;
                }
                b'1' => {
                    val += place * 1;
                    rep[last - i] = 1;
                }
                b'0' => {
                    rep[last - i] = 0;
                }
                b'-' => {
                    val += place * -1;
                    rep[last - i] = -1;
                }
                b'=' => {
                    val += place * -2;
                    rep[last - i] = -2;
                }
                _ => panic!("illegal character in input"),
            }
            place = place * 5;
        }

        Snafu {
            dec: val,
            rep,
        }
    }

    pub fn to_string(&self) -> String {
        let mut as_string: Vec<u8> = Vec::new();
        for s in self.rep.iter() {
            match s {
                2 => as_string.push(b'2'),
                1 => as_string.push(b'1'),
                0 => as_string.push(b'0'),
                -1 => as_string.push(b'-'),
                -2 => as_string.push(b'='),
                _ => panic!("not a valid value in rep."),
            }
        }
        String::from_utf8( as_string).unwrap()
    }

    pub fn add(self, rhs: &Self) -> Snafu {
        let mut m: i32 = (self.rep.len() - 1) as i32;
        let mut o: i32 = (rhs.rep.len() - 1)  as i32;
        let mut res: Vec<i8> = vec![0; self.rep.len().max(rhs.rep.len())];
        let mut carry: i8 = 0;
        while m >= 0 || o >= 0 {
            if m >= 0 && o >= 0 {
                let temp = self.rep[m as usize] + rhs.rep[o as usize] + carry;
                if temp > 2 {
                    carry = 1;
                    res[m as usize] = temp - 5
                } else if temp < -2 {
                    carry = -1;
                    res[m as usize] = temp + 5
                } else {
                    carry = 0;
                    res[m as usize] = temp
                }
            } else if m >= 0 {
                let temp = self.rep[m as usize] + carry;
                if temp > 2 {
                    carry = 1;
                    res[m as usize] = temp - 5
                } else if temp < -2 {
                    carry = -1;
                    res[m as usize] = temp + 5
                } else {
                    carry = 0;
                    res[m as usize] = temp
                }
            } else if o >= 0 {
                let temp = rhs.rep[o as usize] + carry;
                if temp > 2 {
                    carry = 1;
                    res[o as usize] = temp - 5
                } else if temp < -2 {
                    carry = -1;
                    res[o as usize] = temp + 5
                } else {
                    carry = 0;
                    res[o as usize] = temp
                }
            }
            m -= 1;
            o -= 1;
        }
        if carry != 0{
            let mut tmp = vec![carry;1];
            tmp.extend(res.iter());
            res = tmp;
        }
        Snafu {
            dec:0,
            rep:res,
        }
    }
}
/*pub fn decimal_to_snafu(val:i64) -> String {
    let mut div: i64 = 1;
    while div < val {
        div *= 5;
    }
    div /= 5;
}*/


#[cfg(test)]
mod test
{
    use crate::day25::day25_model::Snafu;

    #[test]
    pub fn test1() {
        let s1 = Snafu::new("10");
        let s2 = Snafu::new("20");
        let s = s1.add(&s2);
        assert_eq!( s.to_string() ,"1=0".to_string());
    }

    #[test]
    pub fn test2() {
        let s1 = Snafu::new("1=");
        let s2 = Snafu::new("1-");
        let s = s1.add(&s2);
        assert_eq!( s.to_string() ,"12".to_string());
    }

    #[test]
    pub fn test3() {
        let s1 = Snafu::new("2=");
        let s2 = Snafu::new("2-");
        let s = s1.add(&s2);
        assert_eq!( s.to_string() ,"1=2".to_string());
        let t = Snafu::new("1=2");
        assert_eq!(t.dec, 17);
    }

    #[test]
    pub fn test4() {
        let s1 = Snafu::new("11");
        let s2 = Snafu::new("22");
        let s = s1.add(&s2);
        let rse = Snafu::new(&s.to_string());
        println!("{}",s.to_string());
        assert_eq!( rse.dec ,13);
    }

}