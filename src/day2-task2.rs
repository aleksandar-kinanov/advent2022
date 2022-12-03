use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum Action {
    Win,
    Draw,
    Lose
}

fn get_opposite(opponent: &str, action: Action) -> i32 {
    let wins = HashMap::from([
        ("1", "2"),
        ("2", "3"),
        ("3", "1")
    ]);
    let loses = HashMap::from([
        ("1", "3"),
        ("2", "1"),
        ("3", "2")
    ]); 
    match action {
        Action::Win => { wins.get(opponent).unwrap().parse::<i32>().unwrap() + 6 },
        Action::Lose => loses.get(opponent).unwrap().parse::<i32>().unwrap(),
        _ => { opponent.parse::<i32>().unwrap() + 3 }
    }
}
fn main() {
    let mut result = 0;
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    for item in data
        .replace(" ", ",")
        .replace("A", "1")
        .replace("B", "2")
        .replace("C", "3")
        .split("\n")
    {      
        let split_data: Vec<&str> = item.split(",").collect();

        result += match split_data[1] {
            "X" => get_opposite(split_data[0], Action::Lose),
            "Y" => get_opposite(split_data[0], Action::Draw),
            "Z" => get_opposite(split_data[0], Action::Win),
            _ => 0
        };

        println!("{:?}", result)
    }
}
