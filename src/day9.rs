use std::collections::HashSet;
use itertools::Itertools;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point{
    x: i32,
    y: i32,
}

fn main() {
    let start = Point { x: 0, y: 0 };
    // let mut tail = Point { x: 0, y: 0 };
    let mut marked:HashSet<Point> = HashSet::new();
    let mut rope = vec![start; 10];
    let input = include_str!("dummy_data.txt");
    marked.insert(start);
    let mut action_x = 0;
    let mut action_y = 0;

    for line in input.lines(){
        let (dir, steps) = line.split_once(' ').unwrap();
        for _ in 0..steps.parse::<i32>().unwrap(){
            action_x = 0;
            action_y = 0;
            match dir{
                "R" => {rope[0].y += 1; action_y = 1;},
                "L" => {rope[0].y -= 1; action_y = -1;},
                "U" => {rope[0].x += 1; action_x = 1;},
                "D" => {rope[0].x -= 1; action_x = -1;},
                _ => panic!("Invalid direction"),
            }
    
            for (head, tail) in (0..rope.len()).tuple_windows() {
                let not_adjacent = (rope[head].x - rope[tail].x).abs() > 1 || (rope[head].y - rope[tail].y).abs() > 1;
                if not_adjacent {
                    rope[tail].x += action_x;
                    rope[tail].y += action_y;
                    if tail == rope.len() - 1 {
                        marked.insert(rope[rope.len() - 1]);
                    }
                }
                
            }
            
            
        }
    }
    println!("{:#?}", marked.len());
}