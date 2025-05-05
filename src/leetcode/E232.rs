struct Solution;

struct MyQueue {
    added: Vec<i32>,
    front: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            added: Vec::new(),
            front: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.added.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.empty() {
            panic!("Stack is empty");
        }

        if !self.front.is_empty() {
            return self.front.pop().unwrap();
        }

        while let Some(popped) = self.added.pop() {
            self.front.push(popped);
        }
        self.front.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.front.is_empty() {
            while let Some(popped) = self.added.pop() {
                self.front.push(popped);
            }
        }
        *self.front.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.front.is_empty() && self.added.is_empty()
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
