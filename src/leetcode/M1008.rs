use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

//Definition for a binary tree node.
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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        add_nodes(&preorder).map(|node| Rc::new(RefCell::new(node)))
    }
}

fn add_nodes(preorder: &[i32]) -> Option<TreeNode> {
    if preorder.is_empty() {
        return None;
    }

    let mut node = TreeNode::new(preorder[0]);
    let idx_right = preorder
        .iter()
        .enumerate()
        .find(|&(_i, v)| v > &preorder[0])
        .map(|(i, _)| i)
        .unwrap_or(preorder.len());

    node.left = add_nodes(&preorder[1..idx_right]).map(|node| Rc::new(RefCell::new(node)));
    node.right = add_nodes(&preorder[idx_right..]).map(|node| Rc::new(RefCell::new(node)));

    Some(node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(8);

        let mut left = TreeNode::new(5);
        let mut right = TreeNode::new(10);

        let ll = TreeNode::new(1);
        let lr = TreeNode::new(7);

        let rr = TreeNode::new(12);

        right.right = Some(Rc::new(RefCell::new(rr)));

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            Some(Rc::new(RefCell::new(root))),
            Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12])
        );
    }

    #[test]
    fn test2() {
        let mut root = TreeNode::new(4);
        let left = TreeNode::new(2);

        root.left = Some(Rc::new(RefCell::new(left)));

        assert_eq!(
            Some(Rc::new(RefCell::new(root))),
            Solution::bst_from_preorder(vec![4, 2])
        );
    }
}
