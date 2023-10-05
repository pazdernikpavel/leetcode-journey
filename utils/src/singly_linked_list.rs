// Simple definition for singly-linked list from LeetCode.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            next,
            val,
        }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<Self>> {
        vec.iter()
            .rev()
            .fold(None, |next, val| {
                let node = Box::new(ListNode::new(*val, next));
                Some(node)
            })
    }
}