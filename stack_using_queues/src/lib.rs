use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self::default()
    }

    // Push new and shift rest of the queue at the end
    pub fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        for _ in 0..self.queue.len() - 1 {
            let val = self.queue.pop_front().unwrap();
            self.queue.push_back(val);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::MyStack;

    #[test]
    fn should_work() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert!(!stack.empty());
    }
}
