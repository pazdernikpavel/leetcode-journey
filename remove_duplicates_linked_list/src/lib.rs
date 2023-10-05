use utils::singly_linked_list::ListNode;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr = &mut head;
    while let Some(node) = curr {
        while let Some(next) = &mut node.next {
            if node.val == next.val {
                node.next = next.next.take();
            } else {
                break;
            }
        }
        curr = &mut node.next;
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = ListNode::from_vec(vec![1, 1, 2]);
        let expected = ListNode::from_vec(vec![1, 2]);
        assert_eq!(delete_duplicates(input), expected);
    }

    #[test]
    fn example2() {
        let input = ListNode::from_vec(vec![1, 1, 2, 3, 3]);
        let expected = ListNode::from_vec(vec![1, 2, 3]);
        assert_eq!(delete_duplicates(input), expected);
    }
}
