use std::mem;
use std::ptr;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    prev: *mut Node,
    next: *mut Node,
    val: i32,
}

impl Node {
    fn new(val: i32) -> Self {
        Self {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            val,
        }
    }
}

struct MyLinkedList {
    head: *mut Node,
    tail: *mut Node,
    len: i32,
}

impl Default for MyLinkedList {
    fn default() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self::default()
    }

    fn get(&self, index: i32) -> i32 {
        if index >= self.len {
            return -1;
        }

        let mut target = self.head;

        unsafe {
            for _ in 0..index {
                target = (*target).next;
            }

            (*target).val
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_head = Box::into_raw(Box::new(Node::new(val)));
        let old_head = self.head;

        unsafe {
            if self.head.is_null() {
                self.head = new_head;
                self.tail = new_head;
            } else {
                (*old_head).prev = new_head;
                (*new_head).next = old_head;
                self.head = new_head;
            }
        }

        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_tail = Box::into_raw(Box::new(Node::new(val)));
        let old_tail = self.tail;

        unsafe {
            if self.tail.is_null() {
                self.tail = new_tail;
                self.head = new_tail;
            } else {
                (*new_tail).prev = old_tail;
                self.tail = new_tail;
                (*old_tail).next = new_tail;
            }
        }

        self.len += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        match index {
            0 => {
                self.add_at_head(val);
            }
            index if index == self.len => {
                self.add_at_tail(val);
            }
            index if index > self.len => {}
            _ => {
                let new_mid = Box::into_raw(Box::new(Node::new(val)));
                let mut before_node = self.head;

                unsafe {
                    for _ in 0..index {
                        before_node = (*before_node).next;
                    }

                    let prev = (*before_node).prev;

                    (*prev).next = new_mid;
                    (*new_mid).next = before_node;
                    (*new_mid).prev = prev;
                    (*before_node).prev = new_mid;
                }

                self.len += 1;
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        match (index, self.len) {
            // No elements
            (index, len) if index >= len => {}
            // One element
            (0, 1) => {
                let target_node = self.head;
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
                unsafe { mem::drop(Box::from_raw(target_node)) }
                self.len -= 1;
            }
            // Tail element
            (index, len) if index == len - 1 => unsafe {
                let target_node = self.tail;
                let before = (*target_node).prev;
                self.tail = before;
                (*before).next = ptr::null_mut();
                mem::drop(Box::from_raw(target_node));
                self.len -= 1;
            },
            // Head element
            (0, _) => unsafe {
                let target_node = self.head;
                let after = (*target_node).next;
                self.head = after;
                (*after).prev = ptr::null_mut();
                mem::drop(Box::from_raw(target_node));
                self.len -= 1;
            },

            // Rest
            (index, _) => {
                let mut target_node = self.head;
                unsafe {
                    for _ in 0..index {
                        target_node = (*target_node).next;
                    }
                    let before = (*target_node).prev;
                    let after = (*target_node).next;
                    (*before).next = after;
                    (*after).prev = before;
                    mem::drop(Box::from_raw(target_node));
                    self.len -= 1;
                }
            }
        }
    }
}

impl Drop for MyLinkedList {
    fn drop(&mut self) {
        while self.len > 0 {
            self.delete_at_index(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);
        assert_eq!(list.get(1), 2);
        list.delete_at_index(1);
        assert_eq!(list.get(1), 3);
    }
}
