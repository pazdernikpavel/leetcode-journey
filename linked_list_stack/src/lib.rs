#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Self {
            value,
            next,
        }
    }
}

#[derive(Debug)]
pub struct LinkedListStack<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedListStack<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value),
            _ => None
        }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node::new(value, self.head.take())));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.length -= 1;
            Some(node.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_ops() {
        let mut stack = LinkedListStack::new();
        assert_eq!(stack.length, 0);
        assert_eq!(stack.peek(), None);
        stack.push(10);
        assert_eq!(stack.length, 1);
        assert_eq!(stack.peek(), Some(&10));
        stack.push(20);
        assert_eq!(stack.length, 2);
        assert_eq!(stack.peek(), Some(&20));
        stack.push(25);
        assert_eq!(stack.length, 3);
        assert_eq!(stack.peek(), Some(&25));
        assert_eq!(stack.pop(), Some(25));
        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.length, 2);
    }
}
