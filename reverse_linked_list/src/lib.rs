use utils::singly_linked_list::ListNode;

// Time Complexity: O(n)
// Space Complexity: O(1)

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut current) = (None, head);
    while let Some(mut node) = current.take() {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = ListNode::from_vec(vec![1,2,3,4,5]);
        let expected = ListNode::from_vec(vec![5,4,3,2,1]);
        assert_eq!(reverse_list(input), expected);
    }

    #[test]
    fn example2() {
        let input = ListNode::from_vec(vec![1,2]);
        let expected = ListNode::from_vec(vec![2,1]);
        assert_eq!(reverse_list(input), expected);
    }

    #[test]
    fn example3() {
        let input = ListNode::from_vec(vec![]);
        let expected = ListNode::from_vec(vec![]);
        assert_eq!(reverse_list(input), expected);
    }
}
