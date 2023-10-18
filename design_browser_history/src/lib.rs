use std::ptr;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    prev: *mut Node,
    next: *mut Node,
    val: String,
}

impl Node {
    fn new(val: String) -> Self {
        Self {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
            val,
        }
    }
}

#[derive(PartialEq, Debug, Eq)]
struct BrowserHistory {
    head: *mut Node,
    tail: *mut Node,
    curr: *mut Node,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        let new_node = Box::into_raw(Box::new(Node::new(homepage)));
        Self {
            head: new_node,
            tail: new_node,
            curr: new_node,
        }
    }

    fn visit(&mut self, url: String) {
        let new_node = Box::into_raw(Box::new(Node::new(url)));
        unsafe {
            if self.curr.is_null() {
                self.head = new_node;
                self.tail = new_node;
                self.curr = new_node;
            } else {
                let old_node = self.curr;

                // Remove ongoing nodes and pointers
                if !(*old_node).next.is_null() {
                    let mut to_remove = (*old_node).next;
                    while !to_remove.is_null() {
                        let next = (*to_remove).next;
                        drop(Box::from_raw(to_remove));
                        to_remove = next;
                    }
                }

                (*old_node).next = new_node;
                (*new_node).prev = old_node;
                self.tail = new_node;
                self.curr = new_node;
            }
        };
    }

    fn back(&mut self, steps: i32) -> String {
        unsafe {
            let mut curr = self.curr;
            for _ in 0..steps {
                if (*curr).prev.is_null() {
                    break;
                } else {
                    curr = (*curr).prev;
                }
            }
            self.curr = curr;
            (*self.curr).val.to_string()
        }
    }

    fn forward(&mut self, steps: i32) -> String {
        unsafe {
            let mut curr = self.curr;
            for _ in 0..steps {
                if (*curr).next.is_null() {
                    break;
                } else {
                    curr = (*curr).next;
                }
            }
            self.curr = curr;
            (*self.curr).val.to_string()
        }
    }
}

impl Drop for BrowserHistory {
    fn drop(&mut self) {
        if !self.head.is_null() {
            unsafe {
                let mut curr_head = self.head;
                while !curr_head.is_null() {
                    let next = (*curr_head).next;
                    drop(Box::from_raw(curr_head));
                    curr_head = next;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let mut history = BrowserHistory::new(String::from("leetcode.com"));
        history.visit(String::from("google.com"));
        history.visit(String::from("facebook.com"));
        history.visit(String::from("youtube.com"));
        assert_eq!(history.back(1), String::from("facebook.com"));
        assert_eq!(history.back(1), String::from("google.com"));
        assert_eq!(history.forward(1), String::from("facebook.com"));
        history.visit(String::from("linkedin.com"));
        assert_eq!(history.forward(2), String::from("linkedin.com"));
        assert_eq!(history.back(2), String::from("google.com"));
        assert_eq!(history.back(7), String::from("leetcode.com"));
    }
}
