extern crate advent_of_code_2018;

#[macro_use] 
extern crate scan_fmt;

use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::io::prelude::*;

struct Star {
    pos: (i32,i32),
    vel: (i32,i32),
}

fn input_stars(input: std::io::BufReader<&File>) -> Vec<Star> {
    let mut stars = Vec::new();
    for line in input.lines() {
        let line = String::from(line.unwrap());
        if let (Some(xpos),Some(ypos),Some(xvel),Some(yvel)) = scan_fmt!(&line,"position=< {}, {}> velocity=< {}, {}>",i32, i32,i32,i32) {
            stars.push(Star{pos: (xpos,ypos), vel: (xvel,yvel)})
        }
    }
    stars
}

fn calc_y_distance(stars: &Vec<Star>) -> i32 {
    let min_y_distance = stars.iter().min_by(|&star1, &star2| star1.pos.1.cmp(&star2.pos.1)).unwrap();
    let max_y_distance = stars.iter().max_by(|&star1, &star2| star1.pos.1.cmp(&star2.pos.1)).unwrap();
    max_y_distance.pos.1 - min_y_distance.pos.1
}

fn print_stars(stars: &Vec<Star>) ->() {
    let mut rows = HashMap::new();
    let y_shift = stars.iter().min_by(|&star1, &star2| star1.pos.1.cmp(&star2.pos.1)).unwrap().pos.1;
    let y_max = stars.iter().max_by(|&star1, &star2| star1.pos.1.cmp(&star2.pos.1)).unwrap().pos.1 - y_shift;
    let x_shift = stars.iter().min_by(|&star1, &star2| star1.pos.0.cmp(&star2.pos.0)).unwrap().pos.0;
    let x_max = stars.iter().max_by(|&star1, &star2| star1.pos.0.cmp(&star2.pos.0)).unwrap().pos.0 - x_shift;
    // let max_x =  stars.iter().max_by(|&star1, &star2| star1.pos.0.cmp(&star2.pos.0)).unwrap().pos.0 - x_shift;
    for star in stars.iter() {
        rows.entry(star.pos.1 - y_shift).or_insert(Vec::new()).push(star.pos.0 - x_shift);
    }
    for y_val in 0..y_max+1{
        let mut row_str = (0..x_max+1).map(|_| " ").collect::<String>();
        if let Some(x_vec) = rows.get_mut(&y_val) {
            x_vec.sort();
            for x_val in x_vec.iter() {
                row_str.remove(*x_val as usize);
                row_str.insert(*x_val as usize,'#');
            }
        }
        println!("{}", row_str);
    }
}


fn find_min_y_generation(mut stars:  Vec<Star>) -> u32{
    let mut gen = 0;
    let mut y_dis_cur = calc_y_distance(&stars);
    let mut y_dis_old = y_dis_cur.clone();

    while y_dis_old >= y_dis_cur {
        gen = gen +1;
        for mut star in stars.iter_mut() {
            star.pos.0 = star.pos.0 + star.vel.0;
            star.pos.1 = star.pos.1 + star.vel.1;
        }
        y_dis_old = y_dis_cur;
        y_dis_cur = calc_y_distance(&stars)
    }
    //above loop goes one to far
    for mut star in stars.iter_mut() {
        star.pos.0 = star.pos.0 - star.vel.0;
        star.pos.1 = star.pos.1 - star.vel.1;
    }
    print_stars(&stars);
    gen-1
}


fn main() {
    let f = File::open("./data/day10.txt").expect("file not found");
    let file = BufReader::new(&f);
    let input = input_stars(file);
    let gen = find_min_y_generation(input);

    println!("{}",gen);
}