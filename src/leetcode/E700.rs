struct Solution;

use std::cell::RefCell;
use std::cmp::Ordering::*;
use std::rc::Rc;

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
