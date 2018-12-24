extern crate advent_of_code_2018;

use std::fs::File;
use std::io::prelude::*;

const RACK_OFFSET: i32 = 10;
const GRID_SIZE: i32 = 300;

fn generate_grid(serial_num: i32) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for x in 0..GRID_SIZE {
        grid.push(Vec::new());
        for y in 0..GRID_SIZE {
            let rack_id = x+RACK_OFFSET;
            grid[x as usize].push( (((((rack_id*y) + serial_num)*rack_id)/100)%10) - 5);
        }
    }
    grid
}


fn find_max_power(grid:  &Vec<Vec<i32>>, square_size: usize) ->(usize,usize,i32){
    let mut partial_row_sums: Vec<Vec<i32>> = Vec::new();
    for x in 0..(grid.len()) { 
        partial_row_sums.push(Vec::new());
        for y in 0..(grid[x].len() - square_size){
            let mut partial_sum = 0;
            for size in 0 .. square_size {
                partial_sum += grid[x][y+ size];
            }
            partial_row_sums[x].push(partial_sum);
        }
    }
    let mut powers = Vec::new();
    for column in 0 .. partial_row_sums.len() - square_size{
        for cur_row_idx in 0 .. partial_row_sums[column].len() {
            let mut sum = 0;
            for size in 0 .. square_size {
                sum += partial_row_sums[column + size][cur_row_idx];
            }
            powers.push((column,cur_row_idx, sum));
        }
    }
    *powers.iter().max_by(|x,y| x.2.cmp(&y.2)).unwrap()
}

fn main() {
    let mut f = File::open("./data/day11.txt").expect("file not found");
    let mut serial_str = String::from("");
    let _ = f.read_to_string(&mut serial_str);
    let serial_num = serial_str.parse::<i32>().unwrap();
    let power_grid = generate_grid(serial_num);
    let answer = find_max_power(&power_grid,3);
    println!("(x: {}, y: {}) , value:{}", answer.0, answer.1 , answer.2);

    let mut powers = Vec::new();
    // takes probably 3 minutes
    for x in 1..GRID_SIZE {
        println!("x = {}",x);
        powers.push(find_max_power(&power_grid,x as usize));
    }
    let (square,answer2) = powers.iter().enumerate().max_by(|(_,x),(_,y)| x.2.cmp(&y.2)).unwrap();
    println!("(x: {}, y: {}) ,value: {}, size:{}", answer2.0, answer2.1 ,answer2.2, square);
}