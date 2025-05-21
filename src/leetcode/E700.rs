use crate::leetcode::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering::*;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current = root;

        while let Some(node) = current {
            let borrowed = node.borrow();
            match val.cmp(&borrowed.val) {
                Equal => return Some(node.clone()),
                Less => current = borrowed.left.clone(),
                Greater => current = borrowed.right.clone(),
            }
        }

        None
    }
}
