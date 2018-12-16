extern crate advent_of_code_2018;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn make_tree(input: &mut impl Iterator<Item=usize>) -> Node {
    let num_child_node = input.next().unwrap();
    let num_metadata = input.next().unwrap();
    Node {
        children: (0..num_child_node).map(|_| make_tree(input)).collect(),
        metadata: input.take(num_metadata).collect(),
    }
}

fn sum_metadata(root: &Node) ->usize {
    root.metadata.iter().sum::<usize>() + root.children.iter().map(sum_metadata).sum::<usize>()
}

fn find_node_value(root: &Node) -> usize {
    if root.children.is_empty() {
        root.metadata.iter().sum()
    }
    else {
        root.metadata.iter().map(|data| match root.children.get(*data-1) {
                                    Some(node) => return find_node_value(node),
                                    None => return 0,
                                    }).sum()
    }
}

fn main() {
    let mut f = File::open("./data/day8.txt").expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let mut input_as_nums = input.split(" ").filter_map(|l| l.parse::<usize>().ok());
    let tree_root = make_tree(&mut input_as_nums);
    let answer_part_1 = sum_metadata(&tree_root);
    println!("{}", answer_part_1);
    let answer_part_2 = find_node_value(&tree_root);
    println!("{}",answer_part_2);
}