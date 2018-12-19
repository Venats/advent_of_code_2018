extern crate advent_of_code_2018;

#[macro_use] 
extern crate scan_fmt;

use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

const SPECIAL_MARBLES: usize = 23;

fn max_score(num_players: usize, max_marble: usize) -> usize{
    let mut board = VecDeque::with_capacity(max_marble);
    let mut players : Vec<usize> = vec![0; num_players];
    let mut player_turn = 0;
    board.push_back(0);
    for marble in 1..max_marble+1 {
        if marble % SPECIAL_MARBLES == 0 {
            for _ in 0..7 {
                let old_marble = board.pop_back().unwrap();
                board.push_front(old_marble);
            }
            players[player_turn] += marble + board.pop_front().unwrap();
        }
        else {
            for _ in 0..2 {
                let old_marble = board.pop_front().unwrap();
                board.push_back(old_marble);
            }
            board.push_front(marble);
        }
        player_turn = (player_turn +1) % num_players;
    }
    *players.iter().max().unwrap()
}


fn main() {
    let mut f = File::open("./data/day9.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    if let (Some(num_players),Some(max_marble)) = scan_fmt!(&input,"{} players; last marble is worth {} points",usize, usize) {
        println!("{}", max_score(num_players, max_marble));
        println!("{}", max_score(num_players, max_marble*100));
    }
}