extern crate advent_of_code_2018;

#[macro_use] 
extern crate scan_fmt;

use advent_of_code_2018::*;
use std::fs::File;
use std::io::BufReader;

const NUM_TASKS: usize = 26;
const BASE_TIME: usize = 60;
const NUM_WORKERS : usize = 5;

#[derive(Clone,Debug)]
struct TaskGraph {
    edges: [[usize;NUM_TASKS];NUM_TASKS ] 
}

impl TaskGraph {
    pub fn new() ->TaskGraph{
        TaskGraph {
            edges: [[0;NUM_TASKS];NUM_TASKS],
        }
    }
    pub fn add_edge(&mut self,task: char, req : char) {
        let task_idx = task as usize - 'A' as usize;
        let req_idx = req as usize - 'A' as usize;
        self.edges[task_idx][req_idx] = 1;
    }
    pub fn task_has_no_req(&self, task_idx: usize ) -> bool {
        self.edges[task_idx].iter().fold(true, |acc, &req| acc && (req == 0))
    }

    pub fn complete_task(&mut self, req_idx: usize) -> () {
        for mut task in self.edges.iter_mut() {
            task[req_idx] = 0;
        }
    }

    pub fn worked(&self, task_idx: usize, workers: &[Worker; NUM_WORKERS]) ->bool{
        workers.iter().fold(false, |acc, worker| acc || worker.task_idx == task_idx && !worker.is_free())
    }
}

#[derive(Clone,Copy, Debug)]
struct Worker {
    time_left: usize,
    task_idx: usize,
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            time_left: 0,
            task_idx: 0,
        }
    }
    pub fn assign_worker(&mut self, task_idx: usize) -> (){
        self.time_left = BASE_TIME + task_idx+1;
        self.task_idx = task_idx;
    }
    pub fn is_free(&self) -> bool {
        self.time_left == 0
    }
    pub fn work(&mut self) -> () {
        if self.time_left > 0 {
            self.time_left -= 1;
        }
    }
}

fn string_to_tasks(input_string: &Vec<String>) -> TaskGraph {
    let mut task_graph = TaskGraph::new();
    for line in input_string.iter() {
        if let (Some(req_name),Some(task_name)) = scan_fmt!(line,"Step {} must be finished before step {} can begin",char, char) {
            task_graph.add_edge(task_name,req_name);
        }
    }
    task_graph
}


fn find_order(tasks: &mut TaskGraph) -> Vec<char>{
    let mut order = Vec::new();
    while let Some((complete_idx,_)) = tasks.edges.iter()
                                                .enumerate()
                                                .find(|&(idx,_)| !order.contains(&idx) && tasks.task_has_no_req(idx)) {
        order.push(complete_idx);
        tasks.complete_task(complete_idx);
    }
    //kind of ugly
    order.iter().map(|&idx| (idx + 'A' as usize) as u8 as char).collect()
}


fn find_order_workers(tasks: &mut TaskGraph) -> (usize, Vec<char>) {
    let mut order: Vec<usize> = Vec::new();
    let mut time = 0;
    let mut workers = [Worker::new(); NUM_WORKERS];

    while order.len() < NUM_TASKS {
        let task_idxs: Vec<usize> = tasks.edges.iter()
                                    .enumerate()
                                    .filter(|&(idx,_)| !order.contains(&idx) && tasks.task_has_no_req(idx) && !tasks.worked(idx, &workers))
                                    .map(|(idx, _)| idx)
                                    .collect();
        for task_idx in task_idxs {
            if let Some(mut worker) = workers.iter_mut().find(|worker| worker.is_free()){
                worker.assign_worker(task_idx);
            }
        }
        time += 1;
        for worker in workers.iter_mut().filter(|worker| !worker.is_free()) {
            worker.work();
            if worker.is_free() {
                order.push(worker.task_idx);
                tasks.complete_task(worker.task_idx);
            }
        }
    }

    (time, order.iter().map(|&idx| (idx + 'A' as usize) as u8 as char).collect())
}


fn main() {
    let  f = File::open("./data/day7.txt").expect("file not found");
    let file = BufReader::new(&f);
    let input_string = read_input_as_string(file).unwrap();
    let mut tasks = string_to_tasks(&input_string);
    let mut tasks_part2 = tasks.clone();
    let ordered_tasks = find_order(&mut tasks);
    for task in ordered_tasks {
        print!("{}",task);
    }
    print!("\n");

    let (time, ordered_tasks_workers) = find_order_workers(&mut tasks_part2);
    print!("Time: {}, ", time);
    for task in ordered_tasks_workers {
        print!("{}",task);
    }
    print!("\n");
}