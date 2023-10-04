// Simple definition for singly-linked list from LeetCode.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// Time Complexity: O(n)
// Space Complexity: O(1)

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut next = head;
    let mut result = 0;
    while let Some(node) = next {
        result = result * 2 + node.val;
        next = node.next;
    }
    result
}

// Note:
//  - + adding a binary value could be done with |= bitwise OR operator
//  - *2 Bit shifting could be done with <<= left shift operator

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut head = Box::new(ListNode::new(1));
        let mut element = Box::new(ListNode::new(0));
        let tail = Box::new(ListNode::new(1));

        element.next = Some(tail);
        head.next = Some(element);

        assert_eq!(get_decimal_value(Some(head)), 5);
    }

    #[test]
    fn example2() {
        let head = Box::new(ListNode::new(0));
        assert_eq!(get_decimal_value(Some(head)), 0);
    }
}
