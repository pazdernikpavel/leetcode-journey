use std::cell::RefCell;
use std::cmp::min;
use std::rc::{Rc, Weak};

// pub struct MinStack {
//     values: Vec<i32>,
//     min_history: Vec<i32>,
// }
//
// impl MinStack {
//     pub fn new() -> Self {
//         MinStack {
//             values: vec![],
//             min_history: vec![],
//         }
//     }
//
//     pub fn push(&mut self, val: i32) {
//         self.values.push(val);
//         if let Some(&last) = self.min_history.last() {
//             if last >= val {
//                 self.min_history.push(val);
//             }
//         } else {
//             self.min_history.push(val);
//         }
//     }
//
//     pub fn pop(&mut self) {
//         let removed = self.values.pop();
//         if let (Some(removed_val), Some(&last_min_val)) = (removed, self.min_history.last()) {
//             if removed_val == last_min_val {
//                 self.min_history.pop();
//             }
//         }
//     }
//
//     pub fn top(&self) -> i32 {
//         self.values[self.values.len() - 1]
//     }
//
//     pub fn get_min(&self) -> i32 {
//         self.min_history[self.min_history.len() - 1]
//     }
// }

// Time Complexity: O(1) - All operations
// Space Complexity: O(n)

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
            // Update new node
            let mut borrowed_node = new_node.borrow_mut();
            borrowed_node.min = min(tail_node.borrow().min, val);
            borrowed_node.prev = Some(Rc::downgrade(tail_node));
            // Adjust stack tail
            tail_node.borrow_mut().next = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
        }
    }

    pub fn pop(&mut self) {
        if let Some(tail_node) = self.tail.as_ref() {
            let prev = tail_node.borrow_mut().prev.take();
            self.tail = prev.and_then(|weak| weak.upgrade()).map(|tail| {
                tail.borrow_mut().next = None;
                tail
            });
        }
    }

    pub fn top(&self) -> i32 {
        // unwrap because this op will never be called on an empty stack
        self.tail.as_ref().unwrap().borrow().val
    }

    pub fn get_min(&self) -> i32 {
        // unwrap because this op will never be called on an empty stack
        self.tail.as_ref().unwrap().borrow().min
    }
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
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
