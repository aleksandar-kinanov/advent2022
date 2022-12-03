use std::fs;
use std::cmp::Ordering;

    fn get_winner(data: Vec::<&str>) -> i32 {
        println!("{:?} - {:?}", data[0], data[1]);
        let thing1 = data[0].parse::<i32>().unwrap();
        let thing2 = data[1].parse::<i32>().unwrap();
        match (thing2 > thing1 && thing2 - thing1 == 2, thing2 > thing1 || thing1 > thing2 && thing1 - thing2 == 2, thing1 == thing2) {
            (true, false, false) => thing2,
            (false, true, false) => 6 + thing2,
            (false, false, true) => 3 + thing2,
            _ => thing2
        }
    }
fn main() {
    let mut result = 0;
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    for item in data
        .replace(" ", ",")
        .replace("X", "A")
        .replace("Y", "B")
        .replace("Z", "C")
        .replace("A", "1")
        .replace("B", "2")
        .replace("C", "3")
        .split("\n")
    {      
        let current = get_winner(item.split(",").collect());
        println!("{:?} + {:?}", result, current);
        result += current;
    }
    println!("{:?}", result);
}
