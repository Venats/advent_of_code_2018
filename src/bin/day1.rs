extern crate advent_of_code_2018;
extern crate failure;

use advent_of_code_2018::*;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

enum Errors 
{
    NoRepeat
}

fn find_repeat_freq(frequencies: &Vec<i32>) -> Result<i32, Errors> {
    let mut sum = 0;
    let mut hash = HashSet::new();
    for freq in frequencies.iter().cycle()
    {
        sum += freq;
        match hash.insert(sum) {
            true    => (),
            false   => return Ok(sum),
        };
    }
    Result::Err(Errors::NoRepeat)
}

fn main() {
    let  f = File::open("./data/day1.txt").expect("file not found");
    let file = BufReader::new(&f);
    let input = read_input_as_num(file).unwrap();
    let answer : i32 = input.iter().sum();
    println!("{}", answer);

    let repeat = match find_repeat_freq(&input) {
        Ok(val) => val,
        Err(_) =>0,
    };
    println!("{}",repeat)
}
