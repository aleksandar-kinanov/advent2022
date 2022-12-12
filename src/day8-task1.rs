use ndarray::{Array2, ArrayView};
use std::fmt;

#[derive(Debug, Clone)]
struct Tree {
    height: char,
    is_left_visible: bool,
    is_right_visible: bool,
    is_bottom_visible: bool,
    is_top_visible: bool,
    is_invisible: bool,
}
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Height: {},\n is_left_visible: {}, \n is_right visible: {}, \n is_bottom_visible: {}, \n is_top_visible: {})", self.height, self.is_left_visible, self.is_right_visible, self.is_bottom_visible, self.is_top_visible)
    }
}
impl Default for Tree {
    fn default() -> Self {
        Tree {
            height: '0',
            is_left_visible: true,
            is_right_visible: true,
            is_bottom_visible: true,
            is_top_visible: true,
            is_invisible: false,
        }
    }
}

impl Tree {
    fn new(height: char) -> Tree {
        Tree {
            height,
            is_left_visible: true,
            is_right_visible: true,
            is_bottom_visible: true,
            is_top_visible: true,
            is_invisible: false,
        }
    }
    fn is_invisible(&self) -> bool {
        !self.is_left_visible
            && !self.is_right_visible
            && !self.is_bottom_visible
            && !self.is_top_visible
    }
}

fn set_visability(
    index: usize,
    current_highest: char,
    use_column: bool,
    reversed: bool,
    value: &mut Tree,
) -> char {
    if index == 0 {
        return value.height;
    }
    if value.height <= current_highest {
        match (use_column, reversed) {
            (false, false) => value.is_left_visible = false,
            (false, true) => value.is_right_visible = false,
            (true, false) => value.is_top_visible = false,
            (true, true) => value.is_bottom_visible = false,
        }
        return current_highest;
    }
    value.height
}

fn iterate_axis(matrix: &mut Array2<Tree>, use_column: bool, reversed: bool) {
    let mut current_highest = '0';
    for mut row in matrix.rows_mut() {
        if reversed {
            for (index, value) in row.iter_mut().rev().enumerate() {
                current_highest = set_visability(index, current_highest, use_column, reversed, value);
            }
        } else {
            for (index, value) in row.indexed_iter_mut() {
                current_highest = set_visability(index, current_highest, use_column, reversed, value);
            }
        }
    }

}
fn main() {
    let mut matrix = Array2::<Tree>::default((0, 99));
    let input = include_str!("dummy_data.txt");
    for item in input.split('\n') {
        let thing: Vec<Tree> = item.chars().map(|x| Tree::new(x)).collect();
        matrix.push_row(ArrayView::from(&thing)).unwrap();
    }

    iterate_axis(&mut matrix, false, false);
    iterate_axis(&mut matrix, false, true);
    // .reversed_axes() moves the object
    let mut mutated_matrix = matrix.reversed_axes();
    iterate_axis(&mut mutated_matrix, true, false);
    iterate_axis(&mut mutated_matrix, true, true);
    let all_tress = mutated_matrix.len();
    let mut invisible_trees = 0;
    for mut item in mutated_matrix.clone().into_iter() {
        if item.is_invisible() {
            item.is_invisible = true;
            invisible_trees += 1;
        }
    }
    let thing = vec![1, 2, 3, 4, 5];
    for item in thing.iter().rev() {
        println!("{}", item);
    }
    println!("{}", all_tress - invisible_trees);
}
