// use std::io::Read;

fn main() {
    let input = include_str!("dummy_data.txt");

    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid[0].len();
    println!(
        "{}",
        grid.iter()
            .enumerate()
            .flat_map(|(tree_position_y, row)| row
                .iter()
                .enumerate()
                .map(move |(tree_position_x, tree_height)| (tree_position_y, tree_position_x, tree_height)))
            .map(|(tree_position_y, tree_position_x, tree_height)| {
                let score = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .map(|(next_tree_position_y, next_tree_position_x)| {
                        let (mut tree_position_y, mut tree_position_x) = (tree_position_y as isize, tree_position_x as isize);
                        let mut visible_trees = 0;
                        let mut last_lower_tree = 0;
                        loop {
                            println!("{} {}", tree_position_y, tree_position_x);
                            tree_position_y += next_tree_position_y;
                            tree_position_x += next_tree_position_x;
                            // if we hit an edge
                            if tree_position_y < 0 || tree_position_x < 0 || tree_position_y as usize >= height || tree_position_x as usize >= width {
                                break;
                            }
                            visible_trees += 1;
                            let next_visible_tree = grid[tree_position_y as usize][tree_position_x as usize];
                            if next_visible_tree > last_lower_tree {
                                if next_visible_tree >= *tree_height {
                                    break;
                                }
                                last_lower_tree = next_visible_tree;
                            }
                        }
                        visible_trees
                    })
                    .product::<u32>();
                score
            })
            .max()
            .unwrap()
    );
}
