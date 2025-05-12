use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        let min = p_val.min(q_val);
        let max = p_val.max(q_val);
        Some(inner(&root.unwrap(), min, max))
    }
}

fn inner(node: &Rc<RefCell<TreeNode>>, min: i32, max: i32) -> Rc<RefCell<TreeNode>> {
    let borrowed = node.borrow();
    let val = borrowed.val;

    if max < val {
        return inner(borrowed.left.as_ref().unwrap(), min, max);
    }

    if min > val {
        return inner(borrowed.right.as_ref().unwrap(), min, max);
    }

    node.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(6);

        let mut left = TreeNode::new(2);
        let mut right = TreeNode::new(8);

        let ll = TreeNode::new(0);
        let mut lr = TreeNode::new(4);

        let rl = TreeNode::new(7);
        let rr = TreeNode::new(9);

        let lrl = TreeNode::new(3);
        let lrr = TreeNode::new(5);

        lr.left = Some(Rc::new(RefCell::new(lrl)));
        lr.right = Some(Rc::new(RefCell::new(lrr.clone())));

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr.clone())));

        right.left = Some(Rc::new(RefCell::new(rl)));
        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left.clone())));
        root.right = Some(Rc::new(RefCell::new(right.clone())));

        let root_node = Some(Rc::new(RefCell::new(root)));

        let p = Some(Rc::new(RefCell::new(left)));
        let mut q = Some(Rc::new(RefCell::new(right)));
        assert_eq!(
            6,
            Solution::lowest_common_ancestor(root_node.clone(), p.clone(), q)
                .unwrap()
                .borrow()
                .val
        );

        q = Some(Rc::new(RefCell::new(lr.clone())));
        assert_eq!(
            2,
            Solution::lowest_common_ancestor(root_node.clone(), p, q)
                .unwrap()
                .borrow()
                .val
        );
    }
}
