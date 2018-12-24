extern crate advent_of_code_2018;

use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Pot {
    Plant,
    Empty,
}

impl Pot {
    pub fn char_to_pot(c: char) -> Pot{
        match c {
            '#' => return Pot::Plant,
            _ => return Pot::Empty,
        };
    }
}

#[derive(Debug, Clone)]
struct PotNHood {
    l1: Pot,
    l2: Pot,
    c: Pot,
    r1: Pot,
    r2: Pot,
}

impl PotNHood {
    pub fn equals(&self, other: &PotNHood) -> bool {
        if self.l1 == other.l1 && self.l2 == other.l2 && self.c == other.c && self.r1 == other.r1 && self.r2 == other.r2 {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Transition {
    nhood: PotNHood,
    result: Pot,
}


#[derive(Debug, Clone)]
struct Board {
    plants: HashMap<i32,Pot>,
    transitions: Vec<Transition>,
    min: i32,
    max: i32,
    gen: u32,
}

impl Board {
    pub fn new() -> Board {
        Board {
            plants: HashMap::new(),
            transitions: Vec::new(),
            min: 0,
            max: 0,
            gen: 0,
        }
    }

    pub fn str_to_state(&mut self, input_str: &str){
        self.min = 0;
        self.max = (input_str.len() -1) as i32;
        for (idx, plant) in input_str.chars().enumerate(){
            match plant {
                '#' => self.plants.insert(idx as i32,Pot::Plant),
                _   => None,
            };
        }
    }

    pub fn advance_to_gen(&mut self, end_gen: u64) {
        for _ in 0..end_gen{
            let mut new_pots = HashMap::new();
            for idx in self.min -2 .. self.max + 2 {
                let nhood = self.get_n_hood(&idx);
                let transition = self.transitions.iter().find(|&trans| trans.nhood.equals(&nhood)).unwrap();
                if transition.result == Pot::Plant {
                    new_pots.insert(idx, Pot::Plant);
                    self.max = std::cmp::max(idx,self.max);
                    self.min = std::cmp::min(idx,self.min);
                }
            }
            self.plants = new_pots;
            self.gen += 1;
        }
    }

    fn get_pot_state(&self, idx: &i32) -> Pot {
        if self.plants.contains_key(idx) {
            return  Pot::Plant;
        }
        Pot::Empty
    }
    fn get_n_hood(&self, idx: &i32) -> PotNHood {
        PotNHood {
            l1: self.get_pot_state(&(*idx -1)),
            l2: self.get_pot_state(&(*idx -2)),
            c:  self.get_pot_state(idx),
            r1: self.get_pot_state(&(*idx + 1)),
            r2: self.get_pot_state(&(*idx + 2)),
        }
    }
}

fn input_board(input: std::io::BufReader<&File>) -> Board {
    let mut board = Board::new();
    let mut line_iter: Vec<String> = input.lines().map(|l| l.unwrap()).collect();
    board.str_to_state(&line_iter.remove(0)[15..]);
    line_iter.remove(0);
    for line in line_iter.iter_mut(){
        let characters: Vec<char> = line.chars().filter(|&c| c == '#' || c == '.').collect();
        board.transitions.push(Transition { nhood: PotNHood{l1: Pot::char_to_pot(characters[1])
        , l2:  Pot::char_to_pot(characters[0])
        , c:  Pot::char_to_pot(characters[2])
        , r1:  Pot::char_to_pot(characters[3])
        , r2:  Pot::char_to_pot(characters[4])
        }
        , result: Pot::char_to_pot(characters[5])
        } );
    }
    board
}

//for part 2 we just notice that after a certain point the sum just goes up by 34, and use that fact to find the answer
fn main() {
    let f = File::open("./data/day12.txt").expect("file not found");
    let  file = BufReader::new(&f);
    let mut board = input_board(file);
    board.advance_to_gen(20);
    println!("{}",  board.plants.iter().map(|(&idx,_)| idx).sum::<i32>());
}