
// #[derive(Debug)]
// struct Queue<T> {
//     queue: Vec<T>,
// }

// impl<T> Queue<T> {
//     fn new() -> Self {
//         Queue { queue: Vec::new() }
//     }

//     fn enqueue(&mut self, item: T) {
//         self.queue.push(item);
//     }
//     fn length(&self) -> usize {
//         self.queue.len()
//     }

//     fn dequeue(&mut self) -> Option<T> {
//         if self.queue.is_empty() {
//             None
//         } else {
//             Some(self.queue.remove(0))
//         }
//     }
// }
// impl <T>Iterator for Queue<T> {
//     // we will be counting with usize
//     type Item = T;

//     // next() is the only required method
//     fn next(&mut self) -> Option<Self::Item> {
//         // Increment our count. This is why we started at zero.
//         let counter = 0;

//         if counter < self.length(){
//             Some(self.queue[counter])
//         } else {
//             None
//         }
//         // Check to see if we've finished counting or not.
//         // if self.count < 6 {
//         //     Some(self.count)
//         // } else {
//         //     None
//         // }
//     }
// }

use std::collections::HashSet;

fn get_result(part: i32, input: &str) -> usize  {
    let number_of_elems: usize;
    if part == 1 {
        number_of_elems = 4;
    } else {
        number_of_elems = 14;
    }
    let mut parsed_data: Vec<char> = input.chars().collect();
    let mut initial_values = parsed_data[0..number_of_elems].to_vec();

    parsed_data.drain(0..number_of_elems);
    
    let mut counter = number_of_elems - 1;
    for char in parsed_data {
        counter += 1;
        let hash_set: HashSet<char> = initial_values.iter().cloned().collect();
        if hash_set.len() == number_of_elems {
            break;
        } else {
            initial_values.remove(0);
            initial_values.push(char);
        }
    }
    counter
}

fn main() {
    let input = include_str!("./dummy_data.txt");
    println!("Part 1: {}", get_result(1, input));
    println!("Part 2: {}", get_result(2, input));
}
