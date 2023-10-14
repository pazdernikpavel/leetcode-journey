use std::cell::RefCell;
use std::cmp::min;
use std::rc::{Rc, Weak};

// TODO: https://leetcode.com/problems/min-stack/

#[derive(Debug)]
pub struct Node {
    val: i32,
    min: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct MinStack {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl Default for MinStack {
    fn default() -> Self {
        MinStack::new()
    }
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node {
            val,
            min: val,
            next: None,
            prev: None,
        }));

        if self.tail.is_none() {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
        } else if let Some(ref mut tail_node) = self.tail {
            new_node.borrow_mut().min = min(tail_node.borrow().min, val);
            tail_node.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(Rc::downgrade(tail_node));
            self.tail = Some(Rc::clone(&new_node));
        }
    }

    // pub fn pop(&mut self) {}

    //
    // pub fn top(&self) -> i32 {
    //     self.values[self.values.len() - 1]
    // }
    //
    // pub fn get_min(&self) -> i32 {
    //     self.min_history[self.min_history.len() - 1]
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        println!("{:?}", min_stack);
        // assert_eq!(min_stack.get_min(), -3);
        // min_stack.pop();
        // assert_eq!(min_stack.top(), 0);
        // assert_eq!(min_stack.get_min(), -2);
    }
}
