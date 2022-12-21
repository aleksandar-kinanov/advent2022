

fn part_one(input: &str){
    let mut parsed: &str = "";
    let mut loop_cycles: u32 = 0;
    let mut counter: i32 = 1;
    let mut sum = 1;
    let mut iteretation = 20;
    let mut checpoints: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line {
            "noop" => { parsed = ""; loop_cycles = 1; },
            _ => { parsed = line.split_whitespace().nth(1).unwrap(); loop_cycles = 2; },
        }
        for index in 0..loop_cycles {
            if counter == iteretation {
                checpoints.push(iteretation * sum);
                iteretation += 40;
            }
            if index == 0 {
                counter += 1;
                continue;
            }
            sum += parsed.parse::<i32>().unwrap();
            counter += 1;
        }

    }
    let sum = checpoints.iter().sum::<i32>();
    println!("{:?}", sum);
}

fn part_two(input: &str){
    let mut cursor_position = 0;
    let mut register = 1;
    let mut parsed: &str = "";
    let mut loop_cycles: u32 = 0;
    let mut crt_line: Vec<char> = Vec::new();
    let mut symbol_to_drow = ' ';
    let mut counter = 0;

    for line in input.lines() {
        match line {
            "noop" => { parsed = ""; loop_cycles = 1; },
            _ => { parsed = line.split_whitespace().nth(1).unwrap(); loop_cycles = 2; },
        }
        for index in 0..loop_cycles {
            if [register-1, register, register+1].contains(&cursor_position) {
                symbol_to_drow = '#';
            } else {
                symbol_to_drow = '.';
            }
            if crt_line.len() == 40 {
                println!("{:?} - {}", crt_line, crt_line.len());
                cursor_position = 0;
                // register = 1;
                crt_line.clear();
                symbol_to_drow = '#';
            }
            if index == 0 {
                cursor_position += 1;
                crt_line.push(symbol_to_drow);
                counter += 1;
                if counter == 240 {
                    println!("{:?}", crt_line);
                }
                continue;
            }
            crt_line.push(symbol_to_drow);
            register += parsed.parse::<i32>().unwrap();
            cursor_position += 1;
            counter += 1;
        }

    }
    println!("{}", counter );

}
fn main() {
    let input = include_str!("dummy_data.txt");
    part_two(input);
}
