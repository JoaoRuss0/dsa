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

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            stack: Self::search_leftmost_node(root),
        }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop();
        if node.is_none() {
            return -1;
        }
        let inner = node.unwrap();
        let borrowed = inner.borrow();

        self.stack
            .extend(Self::search_leftmost_node(borrowed.right.clone()));
        borrowed.val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    fn search_leftmost_node(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        let mut current = node;

        while let Some(ref node) = current {
            stack.push(Rc::clone(node));

            let next = {
                let borrowed = node.borrow();
                borrowed.left.as_ref().map(Rc::clone)
            };
            current = next;
        }

        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = TreeNode::new(7);
        let left = TreeNode::new(3);
        let mut right = TreeNode::new(15);
        let rl = TreeNode::new(9);
        let rr = TreeNode::new(20);

        right.left = Some(Rc::new(RefCell::new(rl)));
        right.right = Some(Rc::new(RefCell::new(rr)));

        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut iterator = BSTIterator::new(Some(Rc::new(RefCell::new(root))));
        assert!(iterator.has_next());
        assert_eq!(3, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(7, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(9, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(15, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(20, iterator.next());
        assert!(!iterator.has_next());
    }

    #[test]
    fn test2() {
        let mut root = TreeNode::new(7);
        let mut right = TreeNode::new(15);
        let rl = TreeNode::new(9);
        let rr = TreeNode::new(20);

        right.left = Some(Rc::new(RefCell::new(rl)));
        right.right = Some(Rc::new(RefCell::new(rr)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut iterator = BSTIterator::new(Some(Rc::new(RefCell::new(root))));
        assert!(iterator.has_next());
        assert_eq!(7, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(9, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(15, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(20, iterator.next());
        assert!(!iterator.has_next());
    }

    #[test]
    fn test3() {
        let mut root = TreeNode::new(7);
        let mut right = TreeNode::new(15);
        let rr = TreeNode::new(20);

        right.right = Some(Rc::new(RefCell::new(rr)));
        root.right = Some(Rc::new(RefCell::new(right)));

        let mut iterator = BSTIterator::new(Some(Rc::new(RefCell::new(root))));
        assert!(iterator.has_next());
        assert_eq!(7, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(15, iterator.next());
        assert!(iterator.has_next());
        assert_eq!(20, iterator.next());
        assert!(!iterator.has_next());
    }
}
