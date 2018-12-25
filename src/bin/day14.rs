extern crate advent_of_code_2018;

use std::fs::File;
use std::io::prelude::*;

const FIRST_RECIPE: u64  = 3;
const SECOND_RECIPE: u64 = 7;


struct Elf {
    idx: usize,
    recipe: u64,
}

impl Elf {
    pub fn update_recipe(&mut self, recipes: &Vec<u64>) {
        self.idx = (self.idx + 1 + (self.recipe as usize)) % recipes.len();
        self.recipe = recipes[self.idx];
    }
}


fn to_digits(mut num: u64) -> Vec<u64> {
    let mut digits_vec = Vec::new();
    if num == 0 {
        digits_vec.push(0);
    }
    else {
        while num > 0 {
            digits_vec.push(num % 10);
            num = num/10;
        }
    }
    digits_vec.reverse();
    digits_vec
}

fn create_new_recipes(elf1: &Elf, elf2: &Elf) -> Vec<u64> {
    to_digits(elf1.recipe + elf2.recipe)
}


fn generate_recipes(num_recipes: usize) -> Vec<u64> {
    let mut recipes = Vec::with_capacity(num_recipes);
    let mut elves = vec!(Elf{idx: 0,recipe: FIRST_RECIPE}, Elf{idx: 1,recipe: SECOND_RECIPE});
    recipes.push(FIRST_RECIPE);
    recipes.push(SECOND_RECIPE);
    while recipes.len() < num_recipes {
        let mut new_recipes = create_new_recipes(&elves[0], &elves[1]);
        for recipe in new_recipes.drain(0..) {
            recipes.push(recipe);
        }
        elves.iter_mut().for_each(|elf| elf.update_recipe(&recipes));
    }
    recipes
}

fn find_10_recipes_after(recipe_num: usize) -> u64 {
    let num_to_gen = recipe_num + 11;
    let ten_next_recipes = &generate_recipes(num_to_gen)[recipe_num..recipe_num+10];
    ten_next_recipes.iter().enumerate().fold(0_u64, |acc, (_, value)| acc*10 + value)
}


fn find_recipe_seq_idx(recipe_seq: u64) -> usize{
    let seq_vec = to_digits(recipe_seq);
    let mut seq_idx = 0;
    let mut recipes = Vec::new();
    let mut elves = vec!(Elf{idx: 0,recipe: FIRST_RECIPE}, Elf{idx: 1,recipe: SECOND_RECIPE});
    recipes.push(FIRST_RECIPE);
    recipes.push(SECOND_RECIPE);
    loop {
        let mut new_recipes = create_new_recipes(&elves[0], &elves[1]);
        for recipe in new_recipes.drain(0..) {
            if recipe == seq_vec[seq_idx] {
                seq_idx +=1;
            }
            else {
                seq_idx = 0;
            }
            recipes.push(recipe);
            if seq_idx == seq_vec.len() {
                return recipes.len() - seq_vec.len();
            }
        }
        elves.iter_mut().for_each(|elf| elf.update_recipe(&recipes));
    }
}

fn main() {
    let mut f = File::open("./data/day14.txt").expect("file not found");
    let mut recipe_str = String::from("");
    let _ = f.read_to_string(&mut recipe_str);
    let recipe_num = recipe_str.parse::<usize>().unwrap();
    let answer = find_10_recipes_after(recipe_num);
    println!("{}",answer);
    let answer2 = find_recipe_seq_idx(recipe_num as u64);
    println!("{}", answer2)
}