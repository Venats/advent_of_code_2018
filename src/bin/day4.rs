extern crate advent_of_code_2018;
extern crate itertools;
#[macro_use] 
extern crate scan_fmt;

use advent_of_code_2018::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
#[derive(Clone)]
struct Guard {
    id: usize,
    sleeping : [u32;60],
}
impl PartialEq for Guard {
    fn eq(&self, other: &Guard) -> bool {
        self.id == other.id
    }
}
impl Eq for Guard {}
impl Hash for Guard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Guard {
    pub fn new(id: usize) -> Guard {
        Guard{id:id, sleeping:[0;60]}
    }
    pub fn parse_to_guards(input: &Vec<String>) -> HashMap<usize,Guard> {
        let mut guard_map = HashMap::new();
        let mut line_iter = input.iter();
        let mut cur_guard = 0;
        let mut start_sleep_time = 0;
        while let Some(line) = line_iter.next() {
            if line.contains("Guard") {
                if let Some(guard_id) = scan_fmt!(line,"[{*d}-{*d}-{*d} {*d}:{*d}] Guard #{d} begins shift",usize) {
                    cur_guard = guard_map.entry(guard_id).or_insert(Guard::new(guard_id)).id;
                }
            }
            else if line.contains("falls asleep") {
                if let Some(minute) = scan_fmt!(line, "[{*d}-{*d}-{*d} {*d}:{d}] falls asleep",usize) {
                    start_sleep_time = minute;
                }
            }
            else if line.contains("wakes up") {
                if let Some(minute) = scan_fmt!(line, "[{*d}-{*d}-{*d} {*d}:{d}] wakes up",usize) {
                let mut guard = guard_map.entry(cur_guard).or_insert(Guard::new(cur_guard));
                for sleep_time in start_sleep_time .. minute {
                    guard.sleeping[sleep_time] += 1;
                }
                }
            }
        }
        guard_map
    }

    pub fn sleep_time(&self) -> u32 {
        self.sleeping.iter().fold(0, |total, min| total+min)
    }

    pub fn max_sleep_minute(&self) -> usize {
        self.sleeping.iter().enumerate().map(|(x,y)| (y,x)).max().unwrap().1
    }
    pub fn max_sleep_time(&self) -> u32 {
        *self.sleeping.iter().max().unwrap()
    }
}

fn main() {
    let  f = File::open("./data/day4.txt").expect("file not found");
    let file = BufReader::new(&f);
    let mut input = read_input_as_string(file).unwrap();
    input.sort();
    let guards = Guard::parse_to_guards(&input);
    let sleepiest_guard = guards.iter().max_by_key(|guard| guard.1.sleep_time()).unwrap();
    let answer = sleepiest_guard.1.id * sleepiest_guard.1.max_sleep_minute();
    println!("{}", answer);

    let guard_max_minute = guards.iter().max_by_key(|guard| guard.1.max_sleep_time()).unwrap().1;
    println!("{}", guard_max_minute.id * (guard_max_minute.max_sleep_minute() as usize));

}
