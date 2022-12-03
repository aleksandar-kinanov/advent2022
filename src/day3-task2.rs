use std::fs;

fn main() {
    let mut result = 0;
    let data = fs::read_to_string("./dummy_data.txt").expect("Unable to read file");
    let split_data: Vec<&str> = data.split("\n").into_iter().collect();

    for item in split_data.chunks(3) {
        let thing: Vec<char> = item[0]
            .chars()
            .filter(|a| item[1].contains(*a))
            .filter(|a| item[2].contains(*a))
            .collect();
        let parsed = match thing[0].is_uppercase() {
            true => thing[0] as u32 - 38,
            false => thing[0] as u32 - 96,
        };
        result += parsed;
    }
    println!("{:?}", result);
}
