use std::{cmp, fs};

fn main() {
    let mut result = 0;
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");

    let split_data = data.lines();
    for line in split_data {
        let split: Vec<i32> = line
            .replace("-", ",")
            .split(",")
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        if (split[0]..=split[1]).into_iter().any(|item| {
            (split[2]..=split[3])
                .into_iter()
                .collect::<Vec<i32>>()
                .contains(&item)
        }) {
            result += 1;
        }
    }
    println!("{}", result);
}
