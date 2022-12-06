
#[derive(Debug, Clone)]
struct Queue<T> {
    queue: Vec<T>,
    count: usize,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new(), count: 0 }
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }
    fn length(&self) -> usize {
        self.queue.len()
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }
}
impl <T: Copy> Iterator for Queue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length(){
            Some(self.queue[self.count - 1])
        } else {
            None
        }
    }
}

use std::collections::HashSet;

fn get_result(part: i32, input: &str) -> usize  {
    let number_of_elems: usize;
    if part == 1 {
        number_of_elems = 4;
    } else {
        number_of_elems = 14;
    }
    let mut parsed_data: Vec<char> = input.chars().collect();
    let mut initial_values: Queue<char> = Queue::new();
    for item in parsed_data[0..number_of_elems].to_vec() {
        initial_values.enqueue(item);
    }

    parsed_data.drain(0..number_of_elems);
    
    let mut counter = number_of_elems - 1;
    for item in parsed_data {
        counter += 1;
        let hash_set: HashSet<item> = initial_values.clone().into_iter().collect();
        if hash_set.len() == number_of_elems {
            break;
        } else {
            initial_values.dequeue();
            initial_values.enqueue(item);
        }
    }
    counter
}

fn main() {
    let input = include_str!("./dummy_data.txt");
    println!("Part 1: {}", get_result(1, input));
    println!("Part 2: {}", get_result(2, input));
}
