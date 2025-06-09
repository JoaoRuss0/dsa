use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn run() {
    println!("  ├─ Quest 2 - Tangles Trees");

    let mut path = "input/everybody_codes/echoes_of_enigmatus/Q2/P1.txt";
    let mut input = std::fs::read_to_string(path).unwrap();

    let commands = input.lines().map(Command::parse).collect::<Vec<Command>>();

    let mut commands_iter = commands.iter();

    let first = commands_iter.next().unwrap();
    let mut left = TreeNode::new(first.n_left.clone());
    let mut right = TreeNode::new(first.n_right.clone());

    for c in commands_iter {
        left.add(TreeNode::new(c.n_left.clone()));
        right.add(TreeNode::new(c.n_right.clone()));
    }

    println!(
        "  │  ├─ Part 1: {}{}",
        left.largest_level(),
        right.largest_level()
    );

    //path = "input/everybody_codes/echoes_of_enigmatus/Q2/P2.txt";
    //input = std::fs::read_to_string(path).unwrap();

    //println!("  │  ├─ Part 2: {}", );

    //path = "input/everybody_codes/echoes_of_enigmatus/Q2/P3.txt";
    //input = std::fs::read_to_string(path).unwrap();
    //println!("  │  └─ Part 3: {}", );
}

#[derive(Clone)]
struct TreeNodeDetails {
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
        if self.left.is_some() {
            self.left.as_ref().unwrap().borrow().print();
        }
        print!("[{}, {}] ", self.details.rank, self.details.symbol);
        if self.right.is_some() {
            self.right.as_ref().unwrap().borrow().print();
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
        let mut next = match self.details.rank < node.details.rank {
            true => &mut self.right,
            false => &mut self.left,
        };

        match next {
            Some(n) => n.borrow_mut().add(node),
            None => *next = Some(Rc::new(RefCell::new(node))),
        }
    }
}

struct Command {
    id: usize,
    n_left: TreeNodeDetails,
    n_right: TreeNodeDetails,
}

impl Command {
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

        Command {
            id,
            n_left: TreeNodeDetails {
                rank: l_rank,
                symbol: l_symbol,
            },
            n_right: TreeNodeDetails {
                rank: r_rank,
                symbol: r_symbol,
            },
        }
    }
}
