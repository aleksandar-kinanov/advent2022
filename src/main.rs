use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    pub size: Option<usize>,
    pub is_file: bool,
    pub children: HashMap<String, Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            size: None,
            is_file: false,
            children: HashMap::new(),
            parent: None,
        }
    }
}

fn parse_input(lines: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new()));
    let mut cur_node = Rc::clone(&root);
    for line in lines.lines() {
        let split_data: Vec<&str> = line.split(' ').collect();
        match line.starts_with('$') {
            // Comment for myself - Rc::clone does not actually clone, but creates a new reference to the object specified
            // In the case below, it sends reference to the root node, i.e. goes to the top of the tree
            true => { cur_node = parse_command(line, &split_data, cur_node, Rc::clone(&root)); },
            _ => { insert_child(split_data[0], &cur_node, split_data[1]); },
        };
    }
    root
}


fn insert_child(size_or_dir: &str, cur_node: &Rc<RefCell<Node>>, name: &str) {
    // cur_node is actually the parent for the node to be created in the function

    // Init new subnode (child)
    let child = Rc::new(RefCell::new(Node::new()));
    let mut mut_child = child.try_borrow_mut().unwrap();
    // check if child is file and add required data to it
    if size_or_dir != "dir" {
        mut_child.is_file = true;
        mut_child.size = Some(size_or_dir.parse().unwrap());
    }
    mut_child.parent = Some(Rc::clone(cur_node));
    cur_node
        .try_borrow_mut().unwrap()
        .children
        .insert(name.to_string(), Rc::clone(&child));
}

fn parse_command(line: &str, split_data: &[&str], cur_node: Rc<RefCell<Node>>, root: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    println!("{:?}", cur_node.try_borrow().unwrap().children.keys());
    if line.contains("cd") {
        let folder = split_data[2];
        return match folder {
            // Return parent node
            ".." => Rc::clone(cur_node.try_borrow().unwrap().parent.as_ref().unwrap()),
            // Go back to root
            "/" => root,
            // Get 1 deeper, i.e. inside folder
            _ => Rc::clone(&cur_node.try_borrow().unwrap().children.get(folder).unwrap())
        }
    }
    // Case handles when the comand is 'ls' and does nothing
    return Rc::clone(&cur_node);

}

fn calc_sum<'a>(node: & Node, sizes: &'a mut Vec<usize>) -> (usize, &'a mut Vec<usize>) {
    if node.is_file {
        return (node.size.unwrap(), sizes);
    }
    let sum_c = node
        .children
        .values()
        .map(|child| calc_sum(&child.try_borrow().unwrap(), sizes).0)
        .sum();
    sizes.push(sum_c);
    (sum_c, sizes)
}

fn main() {
    let input = include_str!("dummy_data.txt");
    let root = parse_input(input);
    let mut sizes = vec![];
    let (currently_used, sizes) = calc_sum(&root.try_borrow().unwrap(), &mut sizes);
    let sum: usize = sizes.iter().filter(|x| **x < 100000).sum();
    println!("Part 1: {}", sum);

    let size_to_be_freed = 30000000 - (70000000 - currently_used);
    println!("Part 2: {}", sizes.iter().filter(|x| **x > size_to_be_freed).min().unwrap());
}
