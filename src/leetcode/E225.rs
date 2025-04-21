struct MyStack {
    queue: [i32; 100],
    size: usize,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: [0; 100],
            size: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.queue[self.size] = x;
        self.size += 1;
    }

    fn pop(&mut self) -> i32 {
        let top = self.queue[self.size - 1];
        self.size -= 1;
        top
    }

    fn top(&mut self) -> i32 {
        self.queue[self.size - 1]
    }

    fn empty(&mut self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut stack: MyStack = MyStack::new();

        stack.push(1);
        stack.push(2);

        assert_eq!(2, stack.top());
        assert_eq!(2, stack.pop());
        assert!(!stack.empty());
    }
}
