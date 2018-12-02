extern crate failure; 
use failure::Error;
use std::fs::File;
use std::io::{stdin,BufRead,BufReader};

pub fn read_input_as_num(file:std::io::BufReader<&File>) -> Result<Vec<i32>,Error> {
    // let stdin = std::io::stdin();
    let mut result = Vec::new();
    for line in file.lines() {
        let line = line?;
        result.push(line.parse()?);
    }
    Ok(result)
}

pub fn read_input_as_string(file:std::io::BufReader<&File>) -> Result<Vec<String>,Error> {
    // let stdin = std::io::stdin();
    let mut result = Vec::new();
    for line in file.lines() {
        let line = line?;
        result.push(line.parse()?);
    }
    Ok(result)
}