extern crate advent_of_code_2018;

use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum TrainKind {
    Left,
    Right,
    Up,
    Down,
}

impl TrainKind{
    pub fn char_to_train_kind(character: char) -> Option<TrainKind> {
        let train_kind = match character {
            '^' => Some(TrainKind::Up),
            'v' => Some(TrainKind::Down),
            '<' => Some(TrainKind::Left),
            '>' => Some(TrainKind::Right),
            _ => None,
        };
        train_kind
    }
}


#[derive(Debug, Clone)]
struct Train {
    id: u32,
    kind: TrainKind,
    coords: (usize,usize),
    num_intersections: u32
}

impl PartialEq<Train> for Train {
    fn eq(&self, other: &Train) -> bool {
        self.id == other.id
    }
}


impl Train {
    pub fn advance(&mut self) {
        match self.kind {
            TrainKind::Up    => self.coords = (self.coords.0, self.coords.1 - 1),
            TrainKind::Down  => self.coords = (self.coords.0, self.coords.1 + 1),
            TrainKind::Left  => self.coords = (self.coords.0 - 1, self.coords.1),
            TrainKind::Right => self.coords = (self.coords.0 + 1, self.coords.1), 
        };
    }
    pub fn at_intersection(&mut self) {
        match self.num_intersections % 3 {
            0 => self.turn_left(),
            1 => (),
            2 => self.turn_right(),
            _ => (),
        };
        self.num_intersections += 1;
    }

    pub fn at_for_slash(&mut self) {
        match self.kind {
            TrainKind::Up   =>  self.turn_right(),
            TrainKind::Down =>  self.turn_right(),
            TrainKind::Right => self.turn_left(),
            TrainKind::Left  => self.turn_left(),
        }
    }

    pub fn at_back_slash(&mut self) {
        match self.kind {
            TrainKind::Up   =>  self.turn_left(),
            TrainKind::Down =>  self.turn_left(),
            TrainKind::Right => self.turn_right(),
            TrainKind::Left  => self.turn_right(),
        }
    }

    pub fn turn_left(&mut self) {
        match self.kind {
            TrainKind::Up    => self.kind = TrainKind::Left,
            TrainKind::Down  => self.kind = TrainKind::Right,
            TrainKind::Left  => self.kind = TrainKind::Down,
            TrainKind::Right => self.kind = TrainKind::Up,
        };
    }

    pub fn turn_right(&mut self){
        match self.kind {
            TrainKind::Up    => self.kind = TrainKind::Right,
            TrainKind::Down  => self.kind = TrainKind::Left,
            TrainKind::Left  => self.kind = TrainKind::Up,
            TrainKind::Right => self.kind = TrainKind::Down,
        };
    }
}


#[derive(Debug, Clone)]
enum Track{
    Intersection,
    ForSlash,
    BackSlash,
}


impl Track{
    pub fn char_to_track(character: char) -> Option<Track> {
        let track = match character {
            '+' => Some(Track::Intersection),
            '/' => Some(Track::ForSlash),
            '\\' => Some(Track::BackSlash),
            _ => None,
        };
        track
    }
}


#[derive(Debug, Clone)]
struct Board {
    track_map: HashMap<(usize,usize), Track>,
    trains: Vec<Train>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            track_map: HashMap::new(),
            trains: Vec::new(),
        }
    }

    pub fn find_crash(&mut self) -> (usize,usize) {
        loop {
            self.trains.sort_by(|a,b| {let (x1,y1) = a.coords; let (x2,y2) = b.coords; (y2,x2).cmp(&(y1,x1)) });
            for _ in 0..self.trains.len() {
                let mut train = self.trains.pop().unwrap();
                train.advance();
                if self.trains.iter().find(|t| t.coords == train.coords) == None{
                    match self.track_map.get(&train.coords) {
                        Some(Track::Intersection)       => train.at_intersection(),
                        Some(Track::ForSlash)           => train.at_for_slash(),
                        Some(Track::BackSlash)          => train.at_back_slash(),
                        None => (),
                    };
                    self.trains.insert(0,train);
                }
                else {
                    return train.coords;
                }
            }
        }
    }

    pub fn last_crash(&mut self) -> (usize,usize) {
        while self.trains.len() > 1 {
            self.trains.sort_by(|a,b| {let (x1,y1) = a.coords; let (x2,y2) = b.coords; (y2,x2).cmp(&(y1,x1)) });
            let mut i = 0;
            let num_to_move = self.trains.len();
            while i < num_to_move {
                let mut train = self.trains.pop().unwrap();
                train.advance();
                if self.trains.iter().find(|t| t.coords == train.coords) == None{
                    match self.track_map.get(&train.coords) {
                        Some(Track::Intersection)       => train.at_intersection(),
                        Some(Track::ForSlash)           => train.at_for_slash(),
                        Some(Track::BackSlash)          => train.at_back_slash(),
                        None => (),
                    };
                    self.trains.insert(0,train);
                }
                else {
                    let i_cpy = i;
                    for t_idx in (i_cpy+1)..self.trains.len() {
                        if self.trains[t_idx].coords == train.coords{
                            i +=1;
                        }
                    }
                    self.trains.retain(|t| t.coords != train.coords);
                }
                i += 1;
            }
        }
        self.trains[0].coords
    }
}

fn input_track(input: std::io::BufReader<&File>) -> Board {
    let mut board = Board::new();
    let mut train_id = 0;
    for (row,line) in input.lines().map(|l| l.unwrap()).enumerate() {
        for (col, character) in line.chars().enumerate() {
            match Track::char_to_track(character) {
                Some(track) => board.track_map.insert((col,row), track),
                None => None,
            };
            match TrainKind::char_to_train_kind(character) {
                    Some(train) => {board.trains.push(Train{id:train_id, kind: train, coords: (col,row), num_intersections: 0}); train_id +=1;},
                    None => (),
            };
        }
    }
    board
}

fn main() {
    let f = File::open("./data/day13.txt").expect("file not found");
    let  file = BufReader::new(&f);
    let mut board_part_1 = input_track(file);
    let mut board_part_2 = board_part_1.clone();
    let answer = board_part_1.find_crash();
    println!("x: {}, y: {}",answer.0, answer.1);
    let answer2 = board_part_2.last_crash();
    println!("last_crash: x: {}, y: {}",answer2.0,answer2.1 );
}