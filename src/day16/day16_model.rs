// model types for Day16

use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Room{
    pub valve: String,
    pub flow_rate:i32,
    pub leads_to: Vec<String>,
}

impl Room {
    pub fn new(line: &str) -> Room{
        let a = line.split(";").collect::<Vec<&str>>();
        let flow_rate = a[0].split("=").last().unwrap().parse::<i32>().unwrap();
        let leads_to = a[1].trim().split(" ").skip(4).map(|x| x.split(',').take(1).last().unwrap().to_string()).collect_vec();
        Room{
            valve: a[0][6..8].to_string(),
            flow_rate,
            leads_to
        }
    }
}
