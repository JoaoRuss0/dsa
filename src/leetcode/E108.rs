use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        add_binary_tree_nodes(&nums)
    }
}

fn add_binary_tree_nodes(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let mid = nums.len() / 2;

    let mut node = TreeNode::new(nums[mid]);
    node.left = add_binary_tree_nodes(&nums[..mid]);
    node.right = add_binary_tree_nodes(&nums[mid + 1..]);

    Some(Rc::new(RefCell::new(node)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(0);

        let mut left = TreeNode::new(-3);
        let mut right = TreeNode::new(9);

        let ll = TreeNode::new(-10);

        let rl = TreeNode::new(5);

        left.left = Some(Rc::new(RefCell::new(ll)));
        right.left = Some(Rc::new(RefCell::new(rl)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        assert_eq!(
            Some(Rc::new(RefCell::new(root))),
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
        );
    }
}
