#[macro_use] 
extern crate scan_fmt;
extern crate advent_of_code_2018;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug,Clone,Copy,PartialEq,Hash,Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) ->Point {
        Point{x: x, y: y}
    }

    pub fn man_distance(&self, other: &Point) -> usize {
        ( ((self.x - other.x).abs()) + (self.y - other.y).abs() ) as usize
    }

    pub fn open_neighbourhood(&self) -> [Point;4]{
        let x = self.x;
        let y = self.y;
        [Point::new(x+1,y), Point::new(x-1,y),Point::new(x,y+1),Point::new(x,y-1)]
    }

    pub fn sum_distances(&self, input_points: &Vec<Point>) ->usize {
        input_points.iter().map(|point| self.man_distance(point)).sum()
    }

    pub fn is_finite(&self, input_points: &Vec<Point>) -> bool {
        let (mut px,mut nx,mut py,mut ny) = (false,false,false,false);

        for point in input_points.iter() {
            if(point.x,point.y) == (self.x,self.y) {
                continue
            }
            let dis_x = (self.x - point.x).abs();
            let dis_y = (self.y - point.y).abs();
            if dis_x >= dis_y {
                if point.x > self.x {
                    px = true;
                }
                else {
                    nx = true;
                }
            }
            if dis_y >= dis_x {
                if point.y > self.y{
                    py = true;
                }
                else {
                    ny = true;
                }
            }
        }
        px && nx && py && ny
    }

    pub fn unique_min_man_dis_to(&self, input_points: &Vec<Point> ) -> Option<Point> {
        let mut min_point = input_points.get(0).unwrap();
        let mut min = self.man_distance(min_point);
        for point in input_points.iter().skip(1){
            let dist = self.man_distance(point);
            if dist < min {
                min_point = point;
                min = dist;
            }
        }
        let mut min_count = 0;
        for point in input_points.iter(){
            let dist = self.man_distance(point);
            if dist == min {
                min_count +=1;
            }
        }
        if min_count > 1 {
            return None;
        }
        Some(*min_point)
    }

    fn calculate_area_helper(&self,desired_point: &Point, input_points: &Vec<Point>, visited_points: &mut HashSet<Point>) -> usize {
        visited_points.insert(*self);
        let mut area = 1;
        for point in self.open_neighbourhood().iter() {
            
            match visited_points.get(point){
                Some(_) =>  continue,
                None    => (),
            };
            match point.unique_min_man_dis_to(input_points) {
                Some(p) if p == *desired_point => area += point.calculate_area_helper(desired_point,input_points,visited_points),
                _                     =>  (),
            };
        }
        area
    }

    pub fn calculate_area(&self, input_points: &Vec<Point>) -> usize{
        let mut visited_points = HashSet::new();
        let area = self.calculate_area_helper(self,input_points, &mut visited_points);
        area
    }

    pub fn less_than_10000(&self, input_points: &Vec<Point>) -> usize{
        let mut area = 0;
        for x in 0..1000 {
            for y in 0..1000 {
                if Point::new(x,y).sum_distances(input_points) < 10000 {
                    area += 1;
                }
            }
        }
        area
    }
}



fn max_finite_area(input_points: &Vec<Point>) -> usize {
    input_points.iter()
                .filter(|p| p.is_finite(&input_points))
                .map(|p| p.calculate_area(&input_points))
                .max()
                .unwrap()
}

fn area_less_than_10000(input_points: &Vec<Point>) -> usize {
    input_points.get(0).unwrap().less_than_10000(input_points)
}

fn read_input_as_points(file: std::io::BufReader<&File>) -> Vec<Point> {
    let mut result = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        if let (Some(x),Some(y)) = scan_fmt!(&line,"{d}, {d}",isize,isize){
            result.push(Point::new(x,y));
        }
    }
    result
}


fn main() {
    let f = File::open("./data/day6.txt").expect("file not found");
    let file = BufReader::new(&f);
    let input_points = read_input_as_points(file);
    println!("{}", max_finite_area(&input_points));
    println!("{}",area_less_than_10000(&input_points));
}