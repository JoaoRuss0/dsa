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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traversal = vec![];
        search_node(root, &mut traversal);
        traversal
    }
}

fn search_node(node: Option<Rc<RefCell<TreeNode>>>, searched: &mut Vec<i32>) {
    if let Some(node) = node {
        search_node(node.borrow().left.clone(), searched);
        searched.push(node.borrow().val);
        search_node(node.borrow().right.clone(), searched);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let left_right = TreeNode::new(3);
        let mut right = TreeNode::new(2);
        right.left = Some(Rc::new(RefCell::new(left_right)));

        let mut root = TreeNode::new(1);
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            vec![1, 3, 2],
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(root))))
        );
    }

    #[test]
    fn test2() {
        let rrl = TreeNode::new(9);
        let lrl = TreeNode::new(6);
        let lrr = TreeNode::new(7);
        let mut rr = TreeNode::new(8);
        let ll = TreeNode::new(4);
        let mut lr = TreeNode::new(5);
        let mut l = TreeNode::new(2);
        let mut r = TreeNode::new(3);
        let mut root = TreeNode::new(1);

        rr.left = Some(Rc::new(RefCell::new(rrl)));
        lr.left = Some(Rc::new(RefCell::new(lrl)));
        lr.right = Some(Rc::new(RefCell::new(lrr)));
        r.right = Some(Rc::new(RefCell::new(rr)));
        l.left = Some(Rc::new(RefCell::new(ll)));
        l.right = Some(Rc::new(RefCell::new(lr)));
        root.left = Some(Rc::new(RefCell::new(l)));
        root.right = Some(Rc::new(RefCell::new(r)));

        assert_eq!(
            vec![4, 2, 6, 5, 7, 1, 3, 9, 8],
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(root))))
        );
    }
}
