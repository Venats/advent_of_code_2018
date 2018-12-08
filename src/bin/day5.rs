extern crate advent_of_code_2018;
use std::fs::File;
use std::io::prelude::*;

fn reacting(a: char, b: char) -> bool {
    a.is_ascii_uppercase() && b == a.to_ascii_lowercase()
        || a.is_ascii_lowercase() && b == a.to_ascii_uppercase()
}

fn no_opposite_polarity_size(input: &str, skip: Option<char>) -> usize
{
    input.chars().fold(vec!(), |mut stack, b| match stack.last() {
        _ if skip == Some(b.to_ascii_lowercase()) => stack,
        Some(&c) if reacting(b,c)   => {stack.pop(); stack},
        _                           => {stack.push(b); stack},
    }).len()
}


fn main() {
    let mut f = File::open("./data/day5.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    input = input.chars().filter(|a| a.is_alphabetic()).collect();
    println!("{}", no_opposite_polarity_size(&input, None));
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let min_length = alphabet.chars().map(|a| no_opposite_polarity_size(&input, Some(a))).min();
    println!("{}", min_length.unwrap());
}