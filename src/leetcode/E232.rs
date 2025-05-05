struct Solution;

struct MyQueue {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue { stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        let mut temp = Vec::new();

        while let Some(x) = self.stack.pop() {
            temp.push(x);
        }
        self.stack.push(x);
        while let Some(x) = temp.pop() {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        if !self.empty() {
            return self.stack.pop().unwrap();
        }
        panic!("Stack is empty")
    }

    fn peek(&self) -> i32 {
        if !self.empty() {
            return *self.stack.last().unwrap();
        }
        panic!("Stack is empty")
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut queue: MyQueue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(1, queue.peek());
        assert_eq!(1, queue.pop());
        assert!(!queue.empty());
        assert_eq!(2, queue.peek());
        assert_eq!(2, queue.pop());
        assert!(queue.empty());
    }
}
