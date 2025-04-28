use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        get_deepest_branch(root, 0) as i32
    }
}

fn get_deepest_branch(node: Option<Rc<RefCell<TreeNode>>>, depth: u32) -> u32 {
    if let Some(n) = node {
        let max_left = get_deepest_branch(n.borrow().left.clone(), depth + 1);
        let max_right = get_deepest_branch(n.borrow().right.clone(), depth + 1);
        return max_left.max(max_right);
    }

    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(3);

        let left = TreeNode::new(9);
        let mut right = TreeNode::new(20);

        let rl = TreeNode::new(15);
        let rr = TreeNode::new(7);

        right.left = Some(Rc::new(RefCell::new(rl)));
        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(3, Solution::max_depth(Some(Rc::new(RefCell::new(root)))));
    }
}
