use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traversal = Vec::new();
        traverse(root, &mut traversal);
        traversal
    }
}

fn traverse(node: Option<Rc<RefCell<TreeNode>>>, traversal: &mut Vec<i32>) {
    if let Some(node) = node {
        let node = node.borrow();
        traverse(node.left.clone(), traversal);
        traverse(node.right.clone(), traversal);
        traversal.push(node.val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(1);

        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(3);

        let ll = TreeNode::new(4);
        let mut lr = TreeNode::new(5);

        let lrl = TreeNode::new(6);
        let lrr = TreeNode::new(7);

        let mut rr = TreeNode::new(8);

        let rrl = TreeNode::new(9);

        rr.left = Some(Rc::new(RefCell::new(rrl)));
        right.right = Some(Rc::new(RefCell::new(rr)));
        root.right = Some(Rc::new(RefCell::new(right)));

        lr.left = Some(Rc::new(RefCell::new(lrl)));
        lr.right = Some(Rc::new(RefCell::new(lrr)));
        left.left = Some(Rc::new(RefCell::new(ll)));

        left.right = Some(Rc::new(RefCell::new(lr)));
        root.left = Some(Rc::new(RefCell::new(left)));

        assert_eq!(
            vec![4, 6, 7, 5, 2, 9, 8, 3, 1],
            Solution::postorder_traversal(Some(Rc::new(RefCell::new(root))))
        );
    }
}
