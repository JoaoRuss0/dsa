use crate::everybody_codes::echoes_of_enigmatus::Q2::Command::{Add, Swap};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub fn run() {
    println!("  ├─ Quest 2 - Tangles Trees");

    let mut path = "input/everybody_codes/echoes_of_enigmatus/Q2/P1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();

    let mut commands = input.lines().map(Command::parse).collect::<Vec<Command>>();
    let (left, right) = apply_commands(&commands);

    println!(
        "  │  ├─ Part 1: {}{}",
        left.borrow().largest_level(),
        right.borrow().largest_level()
    );

    path = "input/everybody_codes/echoes_of_enigmatus/Q2/P2.txt";
    input = std::fs::read_to_string(path).unwrap();

    commands = input.lines().map(Command::parse).collect::<Vec<Command>>();
    let (left, right) = apply_commands(&commands);

    println!(
        "  │  ├─ Part 2: {}{}",
        left.borrow().largest_level(),
        right.borrow().largest_level()
    );

    //path = "input/everybody_codes/echoes_of_enigmatus/Q2/P3.txt";
    //input = std::fs::read_to_string(path).unwrap();
    //println!("  │  └─ Part 3: {}", );
}

fn apply_commands(commands: &[Command]) -> (Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>) {
    let left;
    let right;

    let mut additions = HashMap::new();
    let mut swaps = HashMap::new();

    let mut commands_iter = commands.iter();
    let first = commands_iter.next().unwrap();

    match first {
        Add(command) => {
            left = Rc::new(RefCell::new(TreeNode::new(command.left.clone())));
            right = Rc::new(RefCell::new(TreeNode::new(command.right.clone())));

            additions.insert(command.id, command);
        }
        _ => panic!("Invalid first command, cannot swap empty trees"),
    }

    {
        for c in commands_iter {
            match c {
                Add(command) => {
                    left.borrow_mut().add(TreeNode::new(command.left.clone()));
                    right.borrow_mut().add(TreeNode::new(command.right.clone()));

                    additions.insert(command.id, command);
                }
                Swap(id) => match additions.get(id) {
                    Some(addition) => {
                        let swapped = *swaps.get(&id).unwrap_or(&false);

                        let (t_left, t_right) = if swapped {
                            (&right, &left)
                        } else {
                            (&left, &right)
                        };

                        let found_left =
                            TreeNode::find(t_left, addition.left.rank, None, true).unwrap();
                        let found_right =
                            TreeNode::find(t_right, addition.right.rank, None, false).unwrap();

                        {
                            let mut l_borrowed = found_left.node.borrow_mut();
                            let mut r_borrowed = found_right.node.borrow_mut();

                            let mut temp = l_borrowed.left.take();
                            l_borrowed.left = r_borrowed.left.take();
                            r_borrowed.left = temp;

                            temp = l_borrowed.right.take();
                            l_borrowed.right = r_borrowed.right.take();
                            r_borrowed.right = temp;
                        }

                        match (found_left.parent, found_right.parent) {
                            (Some(left), Some(right)) => {
                                let l_node = if found_left.is_left {
                                    &mut left.borrow_mut().left
                                } else {
                                    &mut left.borrow_mut().right
                                };

                                let r_node = if found_right.is_left {
                                    &mut right.borrow_mut().left
                                } else {
                                    &mut right.borrow_mut().right
                                };

                                let temp = l_node.take();
                                *l_node = r_node.take();
                                *r_node = temp;

                                swaps.insert(id, !swapped);
                            }
                            (None, None) => t_left.swap(t_right),
                            _ => panic!("Invalid swap command, cannot swap empty trees"),
                        }
                    }
                    None => panic!("Invalid swap command, no addition with id {} found", id),
                },
            }
        }
    }

    (left, right)
}

#[derive(Clone)]
pub struct TreeNodeDetails {
    rank: usize,
    symbol: char,
}

#[derive(Clone)]
pub struct TreeNode {
    pub details: TreeNodeDetails,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(details: TreeNodeDetails) -> Self {
        TreeNode {
            details,
            left: None,
            right: None,
        }
    }

    fn print(&self) {
        print!("[{:>3}, {:>3}] ", self.details.rank, self.details.symbol);

        let mut layer = VecDeque::new();
        layer.push_back(self.left.clone());
        layer.push_back(self.right.clone());

        while !layer.is_empty() {
            println!();
            let mut next = VecDeque::new();

            while let Some(node_opt) = layer.pop_front() {
                match node_opt {
                    Some(node) => {
                        let borrowed = node.borrow();
                        next.push_back(borrowed.left.clone());
                        next.push_back(borrowed.right.clone());

                        print!(
                            "[{:>3}, {:>3}] ",
                            borrowed.details.rank, borrowed.details.symbol
                        );
                    }
                    None => print!("[   |   ]"),
                }
            }

            layer = next;
        }
    }

    fn largest_level(&self) -> String {
        let mut layer = VecDeque::new();
        layer.push_back(self.left.clone());
        layer.push_back(self.right.clone());

        let mut message = self.details.symbol.to_string();

        while !layer.is_empty() {
            let mut temp = String::new();
            let mut next = VecDeque::new();

            while let Some(node_opt) = layer.pop_front() {
                if node_opt.is_none() {
                    continue;
                }

                let node = node_opt.unwrap();
                let borrowed = node.borrow();
                temp.push(borrowed.details.symbol);

                next.push_back(borrowed.left.clone());
                next.push_back(borrowed.right.clone());
            }

            if temp.len() > message.len() {
                message = temp;
            }

            layer = next;
        }

        message
    }

    fn add(&mut self, node: TreeNode) {
        let next = match self.details.rank < node.details.rank {
            true => &mut self.right,
            false => &mut self.left,
        };

        match next {
            Some(n) => n.borrow_mut().add(node),
            None => *next = Some(Rc::new(RefCell::new(node))),
        }
    }

    fn find(
        node: &Rc<RefCell<TreeNode>>,
        value: usize,
        parent: Option<Rc<RefCell<TreeNode>>>,
        is_left: bool,
    ) -> Option<FoundNode> {
        let borrowed = node.borrow();

        if borrowed.details.rank == value {
            return Some(FoundNode {
                node: Rc::clone(node),
                parent,
                is_left,
            });
        }

        if let Some(left) = &borrowed.left {
            if let Some(r) = TreeNode::find(&Rc::clone(left), value, Some(Rc::clone(node)), true) {
                return Some(r);
            }
        }

        if let Some(right) = &borrowed.right {
            if let Some(r) = TreeNode::find(&Rc::clone(right), value, Some(Rc::clone(node)), false)
            {
                return Some(r);
            }
        }

        None
    }
}

struct FoundNode {
    node: Rc<RefCell<TreeNode>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
    is_left: bool,
}

enum Command {
    Add(AddCommand),
    Swap(usize),
}

impl Command {
    fn parse(c_string: &str) -> Command {
        let split = c_string.split_whitespace().collect::<Vec<&str>>();

        match split[0] {
            "ADD" => AddCommand::parse(c_string),
            _ => Swap(split[1].parse::<usize>().unwrap()),
        }
    }

    fn apply() {
        //let additions = HashMap::new();
    }
}

struct AddCommand {
    id: usize,
    left: TreeNodeDetails,
    right: TreeNodeDetails,
}

impl AddCommand {
    fn parse(c_string: &str) -> Command {
        let split = c_string.split_whitespace().collect::<Vec<&str>>();

        let id = split[1].split("=").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let n_left = split[2].split("=").collect::<Vec<&str>>()[1]
            .split(",")
            .collect::<Vec<&str>>();
        let n_right = split[3].split("=").collect::<Vec<&str>>()[1]
            .split(",")
            .collect::<Vec<&str>>();

        let l_rank = n_left[0][1..].parse::<usize>().unwrap();
        let l_symbol = n_left[1][0..n_left[1].len() - 1].chars().next().unwrap();

        let r_rank = n_right[0][1..].parse::<usize>().unwrap();
        let r_symbol = n_right[1][0..n_right[1].len() - 1].chars().next().unwrap();

        let command = AddCommand {
            id,
            left: TreeNodeDetails {
                rank: l_rank,
                symbol: l_symbol,
            },
            right: TreeNodeDetails {
                rank: r_rank,
                symbol: r_symbol,
            },
        };
        Command::Add(command)
    }
}
