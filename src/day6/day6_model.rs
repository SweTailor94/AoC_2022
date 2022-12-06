// model types for Day6

use std::collections::VecDeque;
use itertools::Itertools;

pub fn part1(input :&String , distinct_count:i32) -> i32 {
    let stream = input.as_bytes();
    let mut index:usize=0;
    let mut last_four: VecDeque<u8> = VecDeque::new();
    while index < stream.len() {
        last_four.push_back(stream[index]);
        if index >= (distinct_count-1) as usize {
            if last_four.iter().unique().count() == distinct_count as usize{
                return index as i32 +1;
            }
            last_four.pop_front();
        }
        index += 1;
    }
    -1
}

#[cfg(test)]
mod test {
    use crate::day6::day6_model::part1;

    #[test]
    pub fn testP1(){
        assert_eq!(part1(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),4),7);
        assert_eq!(part1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),4),5);
        assert_eq!(part1(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(),4),6);
        assert_eq!(part1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),4),10);
        assert_eq!(part1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),4),11);
    }
    #[test]
    pub fn testP2(){
        assert_eq!(part1(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),14),19);
        assert_eq!(part1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),14),23);
        assert_eq!(part1(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(),14),23);
        assert_eq!(part1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),14),29);
        assert_eq!(part1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),14),26);
    }


}
