use utils::singly_linked_list::ListNode;

// TODO: https://leetcode.com/problems/merge-two-sorted-lists/
// Time Complexity: O(?)
// Space Complexity: O(?)

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input_one = ListNode::from_vec(vec![1, 2, 4]);
        let input_two = ListNode::from_vec(vec![1, 3, 4]);
        let expected = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]);
        let output = merge_two_lists(input_one, input_two);
        assert_eq!(expected, output);
    }

    #[test]
    fn test2() {
        let input_one = ListNode::from_vec(vec![]);
        let input_two = ListNode::from_vec(vec![]);
        let expected = ListNode::from_vec(vec![]);
        let output = merge_two_lists(input_one, input_two);
        assert_eq!(expected, output);
    }

    #[test]
    fn test3() {
        let input_one = ListNode::from_vec(vec![0]);
        let input_two = ListNode::from_vec(vec![]);
        let expected = ListNode::from_vec(vec![0]);
        let output = merge_two_lists(input_one, input_two);
        assert_eq!(expected, output);
    }
}
