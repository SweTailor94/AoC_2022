use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use advent_of_code_2022::day16::day16_model::Room;
use advent_of_code_2022::input::get_vector_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_vector_from_file("src/day16/input.txt", parse_input_line);
    println!("Day 16 part 1 ");
    let mut tunnels: HashMap<String, Room> = HashMap::new();
    for i in &input {
        tunnels.insert(i.valve.clone(), i.clone());
    }
    println!("{}", part1(&tunnels));
    println!("Day 16 part 2 ");
    println!("{}", part2(&tunnels));

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Step {
    pub valve: String,
    pub time_left: i32,
    pub accumulated_release: i32,
    pub last: String,
    pub opened: Vec<String>,
}

fn part1(tunnels: &HashMap<String, Room>) -> String {
    let mut best_step = Step {
        valve: "AA".to_string(),
        time_left: 30,
        accumulated_release: 0,
        last: "".to_string(),
        opened: Vec::new(),

    };

    let mut next: VecDeque<Step> = VecDeque::new();
    next.push_back(best_step.clone());
    while next.len() > 0 {
        if let Some(s) = next.pop_front() {
            let (next_tries, done_30_min) = evaluate_next_step(s, &tunnels, best_step.accumulated_release);
            for m in done_30_min {
                if m.accumulated_release > best_step.accumulated_release {
                    best_step = m;
                    println!("Best {}", best_step.accumulated_release);
                }
            }
            for m in &next_tries {
                if m.accumulated_release > best_step.accumulated_release {
                    best_step = m.clone();
                    println!("Best {}", best_step.accumulated_release);
                }
            }
            next.extend(next_tries);
        }
    }

    format!("{:?}", best_step)
}

fn evaluate_next_step(s: Step, tunnels: &HashMap<String, Room>, best_now: i32) -> (Vec<Step>, Vec<Step>) {
    let mut result: Vec<Step> = Vec::new();
    let mut done: Vec<Step> = Vec::new();
    if let Some(room) = tunnels.get(&s.valve) {
        // println!("evaluating {:?}  in {:?}", s, room);

        let mut unopened_valves = tunnels.iter().filter(|(key, room)| !s.opened.contains(key) && room.flow_rate > 0).map(|(k, r)| r.flow_rate).collect_vec();
        unopened_valves.sort_by_key(|v| -v);
        // println!("Unopened valves {:?}",unopened_valves);
        if !s.opened.contains(&s.valve) && room.flow_rate > 0 {
            //spend time open it
            let time_left = s.time_left - 1;
            let accumulated_release = s.accumulated_release + room.flow_rate * time_left;
            let mut opened = s.opened.clone();
            opened.push(s.valve.clone());
            result.push(
                Step {
                    valve: s.valve.clone(),
                    time_left,
                    accumulated_release,
                    last: "".to_string(),
                    opened,
                }
            )
        }

        let mut test_val = s.accumulated_release;
        let mut temp_time = s.time_left - 2;
        for v in unopened_valves {
            test_val += v * temp_time;
            temp_time -= 2;
            if temp_time < 0 {
                break;
            }
        }

        let time_left = s.time_left - 1; // for every possible move
        let accumulated_release = s.accumulated_release;
        if time_left <= 0 || test_val < best_now {
            done.push(s);
        } else {
            let mut found_somewhere_to_go = false;
            for v in &room.leads_to {
                if v != &s.last {
                    result.push(Step {
                        valve: v.clone(),
                        time_left,
                        accumulated_release,
                        last: s.valve.clone(),
                        opened: s.opened.clone(),
                    });
                    found_somewhere_to_go = true;
                }
            }
            if !found_somewhere_to_go {
                done.push(s);
            }
        }
    } else {
        panic!(" Can't find room with valve {}", s.valve);
    }
    (result, done)
}

#[derive(Clone, Debug)]
pub struct Step2 {
    pub me_valve: String,
    pub me_time_left: i32,
    pub me_last: String,
    pub e_valve: String,
    pub e_time_left: i32,
    pub e_last: String,
    pub accumulated_release: i32,
    pub opened: Vec<String>,
}

fn part2(tunnels: &HashMap<String, Room>) -> String {
    let mut start = Step2 {
        me_valve: "AA".to_string(),
        me_time_left: 26,
        me_last: "".to_string(),
        e_valve: "AA".to_string(),
        e_time_left: 26,
        e_last: "".to_string(),
        accumulated_release: 0,
        opened: Vec::new(),

    };
    let mut best: i32 = 0;
    let mut next: VecDeque<Step2> = VecDeque::new();
    next.push_back(start.clone());

    while next.len() > 0 {
        println!("next.len() = {}", next.len());
        if let Some(s) = next.pop_front() {
            let (next_tries, done_30_min) = evaluate_next_step2(s, &tunnels, best);
            for m in done_30_min {
                if m.accumulated_release > best {
                    best = m.accumulated_release;
                    println!("Best {}", best);
                }
            }
            for m in next_tries {
                if m.accumulated_release > best {
                    best = m.accumulated_release;
                    println!("Best {}", best);

                }
                next.push_back(m);
            }
        }
    }

    format!("{:?}", best)
}

fn evaluate_next_step2(s: Step2, tunnels: &HashMap<String, Room>, best_now: i32) -> (Vec<Step2>, Vec<Step2>) {
    println!("Best {}, Evaluating {:?}", best_now, s);
    let mut result: Vec<Step2> = Vec::new();
    let mut done: Vec<Step2> = Vec::new();

    if let Some(my_room) = tunnels.get(&s.me_valve) {
        if let Some(e_room) = tunnels.get(&s.e_valve) {
            let mut unopened_valves = tunnels.iter().filter(|(key, room)| !s.opened.contains(key) && room.flow_rate > 0).map(|(k, r)| r.flow_rate).collect_vec();
            unopened_valves.sort_by_key(|v| -v);
            let mut test_val = s.accumulated_release;
            let mut me_temp_time = s.me_time_left - 2;
            let mut e_tempTime = s.e_time_left - 2;
            let mut i = 0;
            while i < unopened_valves.len() && (me_temp_time > 0 || e_tempTime > 0) {
                if me_temp_time > 0 {
                    test_val += unopened_valves[i] * me_temp_time;
                    i += 1;
                    me_temp_time -= 2;
                }
                if i >= unopened_valves.len() { break; }
                if e_tempTime > 0 {
                    test_val += unopened_valves[i] * e_tempTime;
                    i += 1;
                    e_tempTime -= 2;
                }
            }

            if (s.me_time_left <= 0 && s.e_time_left <= 0) || test_val < best_now {
                println!("Skip branch {:?}", s);
                done.push(s);
            } else {
                // println!("Unopened valves {:?}",unopened_valves);
                if s.me_time_left > 1 && !s.opened.contains(&s.me_valve) && my_room.flow_rate > 0 {
                    //spend time open it
                    let me_time_left = s.me_time_left - 1;
                    let mut accumulated_release = s.accumulated_release + my_room.flow_rate * me_time_left;
                    let mut opened = s.opened.clone();
                    opened.push(s.me_valve.clone());
                    result.push(
                        Step2 {
                            me_valve: s.me_valve.clone(),
                            me_time_left,
                            me_last: "".to_string(),
                            e_valve: s.e_valve.clone(),
                            e_time_left: s.e_time_left,
                            e_last: s.e_last.clone(),
                            accumulated_release,
                            opened: opened.clone(),
                        }
                    );
                    if s.e_time_left > 1 && !opened.contains(&s.e_valve) && e_room.flow_rate > 0 {
                        let e_time_left = s.e_time_left - 1;
                        accumulated_release = accumulated_release + e_room.flow_rate * e_time_left;
                        opened.push(s.e_valve.clone());
                        result.push(
                            Step2 {
                                me_valve: s.me_valve.clone(),
                                me_time_left,
                                me_last: "".to_string(),
                                e_valve: s.e_valve.clone(),
                                e_time_left,
                                e_last: "".to_string(),
                                accumulated_release,
                                opened,
                            }
                        );
                        let mut opened = s.opened.clone();
                        opened.push(s.e_valve.clone());
                        accumulated_release = s.accumulated_release + e_room.flow_rate * e_time_left;
                        result.push(
                            Step2 {
                                me_valve: s.me_valve.clone(),
                                me_time_left: s.me_time_left,
                                me_last: s.me_last.clone(),
                                e_valve: s.e_valve.clone(),
                                e_time_left,
                                e_last: "".to_string(),
                                accumulated_release,
                                opened,
                            }
                        );
                    }
                } else if s.e_time_left > 1 && !s.opened.contains(&s.e_valve) && e_room.flow_rate > 0 {
                    let mut opened = s.opened.clone();
                    opened.push(s.e_valve.clone());
                    let e_time_left = s.e_time_left - 1;
                    let accumulated_release = s.accumulated_release + e_room.flow_rate * e_time_left;
                    result.push(
                        Step2 {
                            me_valve: s.me_valve.clone(),
                            me_time_left: s.me_time_left,
                            me_last: s.me_last.clone(),
                            e_valve: s.e_valve.clone(),
                            e_time_left,
                            e_last: "".to_string(),
                            accumulated_release,
                            opened,
                        }
                    );
                }


                let mut found_somewhere_to_go = false;
                for v in &my_room.leads_to {
                    if v != &s.me_last {
                        for e in &e_room.leads_to {
                            if e != &s.e_last && e != v {
                                result.push(Step2 {
                                    me_valve: v.clone(),
                                    me_time_left: s.me_time_left - 1,
                                    me_last: s.me_valve.clone(),
                                    e_valve: e.clone(),
                                    e_time_left: s.e_time_left - 1,
                                    e_last: s.e_valve.clone(),
                                    accumulated_release: s.accumulated_release,
                                    opened: s.opened.clone(),
                                });
                                found_somewhere_to_go = true;
                            }
                        }
                    }
                }
                if !found_somewhere_to_go {
                    done.push(s);
                }
            }
        }
    } else {
        panic!(" Can't find room with valve {}", s.me_valve);
    }
    (result, done)
}

fn parse_input_line(line: &str) -> Room {
    // Valve AP has flow rate=0; tunnels lead to valves AA, ON
    let r = Room::new(line);
    // println!("{:?}",r);
    r
}
