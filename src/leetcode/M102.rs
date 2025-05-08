use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut traversal = Vec::new();
        let mut layer = Vec::from(vec![root]);

        loop {
            if layer.is_empty() {
                break;
            }

            let mut next_layer = Vec::new();
            let mut values = Vec::new();

            for node in layer {
                if let Some(node_ref) = node {
                    let node = node_ref.borrow();
                    values.push(node.val);
                    next_layer.push(node.left.clone());
                    next_layer.push(node.right.clone());
                }
            }

            if !values.is_empty() {
                traversal.push(values);
            }
            layer = next_layer;
        }

        traversal
    }
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

        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(Some(Rc::new(RefCell::new(root))))
        );
    }
}
