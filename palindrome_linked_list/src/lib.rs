use utils::singly_linked_list::ListNode;

// https://leetcode.com/problems/palindrome-linked-list/
pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    // TODO
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = ListNode::from_vec(vec![1, 2, 2, 1]);
        assert!(is_palindrome(input));
    }

    #[test]
    fn example2() {
        let input = ListNode::from_vec(vec![1, 2]);
        assert!(!is_palindrome(input));
    }
}
