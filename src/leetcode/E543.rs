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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let (biggest_path, biggest_path_between_nodes) = get_diameter(root.unwrap());
        biggest_path.max(biggest_path_between_nodes)
    }
}

fn get_diameter(node: Rc<RefCell<TreeNode>>) -> (i32, i32) {
    let borrowed = node.borrow();

    let mut path_left = (0, 0);
    if let Some(left) = &borrowed.left {
        path_left = get_diameter(Rc::clone(left));
        path_left.0 += 1;
    }

    let mut path_right = (0, 0);
    if let Some(right) = &borrowed.right {
        path_right = get_diameter(Rc::clone(right));
        path_right.0 += 1;
    }

    let biggest_path = (path_left.0).max(path_right.0);
    let biggest_path_between_nodes = path_left
        .1
        .max(path_right.1)
        .max(path_right.0 + path_left.0);

    (biggest_path, biggest_path_between_nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(1);

        let mut left = TreeNode::new(2);
        let right = TreeNode::new(3);

        let ll = TreeNode::new(4);
        let lr = TreeNode::new(5);

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            3,
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))))
        );
    }

    #[test]
    fn test2() {
        let left = TreeNode::new(2);

        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(left)));

        assert_eq!(
            1,
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))))
        );
    }
}
