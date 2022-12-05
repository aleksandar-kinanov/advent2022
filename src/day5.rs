use ndarray::{Array2, ArrayView};

fn move_crate(quantity: u32, from: u32, to: u32, crates: &mut Vec<Vec<&String>>, part: i32) {
    println!("Moving from {} -> {}, {} crates!", from, to, quantity);

    let mut crates_to_move: Vec<&String> = Vec::new();
    // Add all creates that should be moved into a single vector
    for _ in 0..quantity {
        let temp = crates[from as usize].pop().unwrap();
        crates_to_move.push(temp);
    }
    
    if part == 2 {
        crates_to_move.reverse();
    }

    crates[to as usize].append(&mut crates_to_move);
}
fn main() {
    let mut result: Vec<&String> = Vec::new();
    let input = include_str!("./dummy_data.txt");
    let split_data: Vec<&str> = input.split("\n\n").collect();
    let mut crates: Vec<&str> = split_data[0].split("\n").collect();
    let mut parsed_rows: Vec<Vec<&String>> = Vec::new();
    let mut matrix = Array2::<String>::default((9, 0));
    // Remove numbering under columns
    crates.pop();

    // Replace all empty spaces with [0] so a Matrix can be built from the data
    for item in crates {
        let parsed = item
            .replace("    ", " 0")
            .replace(" ", ",")
            .replace("[", "")
            .replace("]", "");
        let column = parsed.split(",").map(String::from).collect::<Vec<String>>();
        // Create Matrix with initial data and use it as column instead of row, i.e. transpose the initial matrix
        matrix.push_column(ArrayView::from(&column)).unwrap();
    }

    // Iterate over matrices rows, remove [0] as it is of no use and add it to vector of vectors to loop over later
    for row in matrix.rows() {
        parsed_rows.push(
            row.into_iter()
                .filter(|x| *x != "0")
                .rev()
                .collect::<Vec<&String>>(),
        );
    }
    
    for line in split_data[1].lines() {
        // refine actions data
        let refined_actions_data = line
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",");
        // Get movement pattern
        let move_actions: Vec<u32> = refined_actions_data
            .split(',')
            .map(|f| f.parse::<u32>().unwrap())
            .collect();
        move_crate(move_actions[0], move_actions[1] - 1, move_actions[2] - 1, &mut parsed_rows, 1)
    }

    for row in parsed_rows {
        result.push(row.clone().pop().unwrap());
    }
    let formatted_result = result.into_iter().map(|x| x.to_owned()).collect::<String>();
    println!("{}", formatted_result);
}
