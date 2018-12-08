extern crate advent_of_code_2018;
extern crate itertools;
#[macro_use] 
extern crate scan_fmt;

use advent_of_code_2018::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Clone,Debug,Copy)]
struct Fabric {
    id: usize,
    left: usize,
    top: usize,
    length: usize,
    width: usize,
}

#[derive(Clone,Debug)]
struct ClaimMap {
    ids: HashSet<usize>,
    fabric_map: HashMap<(usize,usize), Vec<usize>>,
}
impl ClaimMap{
    pub fn new() ->ClaimMap {
        ClaimMap {ids: HashSet::new(), fabric_map: HashMap::new()}
    }
}

impl Fabric {
    pub fn from_string(string: &String) -> Fabric {
        let (id,left,top,width,length) = scan_fmt!(string, "#{d} @ {d},{d}: {d}x{d}",usize, usize,usize,usize,usize);
        Fabric
        {
            id: id.unwrap(),
            left: left.unwrap(),
            top: top.unwrap(),
            length: length.unwrap(),
            width: width.unwrap(),
        }
    }
}


fn make_fabric_map(fabrics: &Vec<Fabric>) -> ClaimMap {
    let mut claim_map = ClaimMap::new();
    for fabric in fabrics.iter() {
        claim_map.ids.insert(fabric.id);
        for x in fabric.left .. (fabric.left+fabric.width) {
            for y in fabric.top .. (fabric.top+fabric.length) {
                claim_map.fabric_map.entry((x,y)).or_insert(Vec::new()).push(fabric.id);
            }
        }
    }
    claim_map
}


fn main() {
    let  f = File::open("./data/day3.txt").expect("file not found");
    let file = BufReader::new(&f);
    let input = read_input_as_string(file).unwrap();
    let fabrics: Vec<Fabric> = input.iter()
                                .map(|x| Fabric::from_string(x))
                                .collect();

    let claim_map = make_fabric_map(&fabrics);
    let (mut id_set,fabric_map) = (claim_map.ids, claim_map.fabric_map);
    let amount_overlap: usize = fabric_map
                                    .values()
                                    .filter(|c| c.len() >1)
                                    .inspect(|overlap| {overlap.iter()
                                                            .for_each(|i| {id_set.remove(i);})})
                                    .count();
    println!("{}", amount_overlap);

    let (id,) = id_set.iter().collect_tuple().unwrap();
    println!("{}", id);
}