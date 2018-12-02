extern crate advent_of_code_2018;

use advent_of_code_2018::*;
use std::fs::File;
use std::io::BufReader;

fn find_checksum(input: &Vec<String>) -> u32 {
    let mut num_2s = 0;
    let mut num_3s = 0;

    for line in input.iter() {
        let mut buckets : [u32; 26] = [0;26];
        for character in line.chars() {
            buckets[(character as usize) - ('a' as usize)] += 1;
        }
        if buckets.iter().find(|&&x| x == 2) != None {
            num_2s +=1;
        }
        if buckets.iter().find(|&&x| x == 3) != None {
            num_3s += 1;
        }
    }

    num_2s * num_3s
}


fn similar_strings(str1 : &str, str2 : &str) -> bool {
    let mut one_difference = false;
    for (char1,char2) in str1.chars().zip(str2.chars()) {
        if char1 != char2 {
            match one_difference {
                false => one_difference = true,
                true => return false,
            };
        }
    }

    true
}


fn get_same_letters(str1 : &str, str2 : &str) -> String {
    let mut matched_chars = String::from("");
    for (char1,char2) in str1.chars().zip(str2.chars()) {
        if char1 == char2 {
            matched_chars.push(char1);
        }
    }
    matched_chars
}


fn related_box_id(input: Vec<String>) -> String {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            if similar_strings(&input[i], &input[j]) {
                return get_same_letters(&input[i], &input[j]);
            }
        }
    }
    String::from("")
}


fn main() {
    let  f = File::open("./data/day2.txt").expect("file not found");
    let file = BufReader::new(&f);
    let input = read_input_as_string(file).unwrap();
    let checksum = find_checksum(&input);
    println!("{}", checksum);

    let box_id = related_box_id(input);
    println!("{}", box_id);
}
