use std::fs;

fn main() {
    let mut result = 0;
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    let split_data: Vec<&str> = data.split("\n").into_iter().collect();

    for item in split_data {
        let len = item.len();
        let first_part = item.split_at(len/2);
        let thing: Vec<char> = first_part.0.chars().filter(|a| first_part.1.contains(*a)).collect();
        let parsed = match thing[0].is_uppercase() {
            true => thing[0] as u32 - 38,
            false => thing[0] as u32 - 96
        };
        
        result += parsed;
    }
    println!("{:?}", result);
}