use utils::singly_linked_list::ListNode;

// Time Complexity: O(n)
// Space Complexity: O(1)

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l), None) | (None, Some(l)) => Some(l),
        (Some(l1), Some(l2)) => match l1.val <= l2.val {
            true => Some(Box::new(ListNode {
                val: l1.val,
                next: merge_two_lists(l1.next, Some(l2)),
            })),
            false => Some(Box::new(ListNode {
                val: l2.val,
                next: merge_two_lists(Some(l1), l2.next),
            })),
        },
    }
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
