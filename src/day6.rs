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