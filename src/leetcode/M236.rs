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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root?;

        let p = p.unwrap();
        let q = q.unwrap();

        let mut p_path: Option<Vec<i32>> = None;
        let mut q_path: Option<Vec<i32>> = None;

        let mut queue = Vec::new();
        queue.push((vec![], root));

        while let Some((path, node)) = queue.pop() {
            let node_inner = node.borrow();
            let mut new_path = path.clone();
            new_path.push(node_inner.val);

            if node_inner.val.eq(&p.borrow().val) {
                p_path = Some(new_path.clone());
            } else if node_inner.val.eq(&q.borrow().val) {
                q_path = Some(new_path.clone());
            } else {
                if node_inner.left.is_some() {
                    queue.push((new_path.clone(), node_inner.left.clone().unwrap()));
                }

                if node_inner.right.is_some() {
                    queue.push((new_path.clone(), node_inner.right.clone().unwrap()));
                }
            }

            if p_path.is_some() && q_path.is_some() {
                break;
            }
        }

        match (p_path, q_path) {
            (Some(p_path), Some(q_path)) => {
                let len = p_path.len().min(q_path.len());

                for i in (0..len).rev() {
                    if p_path[i] == q_path[i] {
                        return Some(Rc::new(RefCell::new(TreeNode::new(q_path[i]))));
                    }
                }
                None
            }
            (Some(_), None) => Some(Rc::new(RefCell::new(TreeNode::new(p.borrow().val)))),
            (None, Some(_)) => Some(Rc::new(RefCell::new(TreeNode::new(q.borrow().val)))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(3);

        let mut left = TreeNode::new(5);
        let mut right = TreeNode::new(1);

        let ll = TreeNode::new(6);
        let mut lr = TreeNode::new(2);

        let rl = TreeNode::new(0);
        let rr = TreeNode::new(8);

        let lrl = TreeNode::new(7);
        let lrr = TreeNode::new(4);

        lr.left = Some(Rc::new(RefCell::new(lrl)));
        lr.right = Some(Rc::new(RefCell::new(lrr)));

        left.left = Some(Rc::new(RefCell::new(ll)));
        left.right = Some(Rc::new(RefCell::new(lr)));

        right.left = Some(Rc::new(RefCell::new(rl)));
        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let root_node = Some(Rc::new(RefCell::new(root)));

        let mut p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let mut q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            Solution::lowest_common_ancestor(root_node.clone(), p, q)
        );

        p = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            Solution::lowest_common_ancestor(root_node, p, q)
        );
    }
}
