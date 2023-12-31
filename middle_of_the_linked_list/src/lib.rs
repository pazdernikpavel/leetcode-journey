use utils::singly_linked_list::ListNode;

// Time Complexity: O(n+1/2n)
// Space Complexity: O(1)w

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut length = 0;

    let mut next = &head;
    while let Some(node) = next {
        length += 1;
        next = &node.next;
    }

    let half_index = if length % 2 == 0 {
        (length / 2) + 1
    } else {
        (length + 1) / 2
    };


    let mut index = 1;
    let mut target = head;

    while index < half_index {
        target = if let Some(node) = target {
            node.next
        } else {
            None
        };
        index += 1;
    }

    target
}

// Super smart solution from LeetCode using 2 pointers and ownership/option magic
//
// pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let (mut fast, mut slow) = (&head, &head);
//     while let Some(ListNode {next: Some(node), ..}) = fast.as_deref() {
//         slow = &slow.as_ref().unwrap().next;
//         fast = &node.next;
//     }
//     slow.clone()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let expected = ListNode::from_vec(vec![3, 4, 5]);
        assert_eq!(middle_node(input), expected);
    }

    #[test]
    fn example2() {
        let input = ListNode::from_vec(vec![1, 2, 3, 4, 5, 6]);
        let expected = ListNode::from_vec(vec![4, 5, 6]);
        assert_eq!(middle_node(input), expected);
    }
}
