use utils::singly_linked_list::ListNode;

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut list_ref = &head;
    let mut counter = 0;
    while let Some(node) = list_ref.as_ref() {
        list_ref = &node.next;
        counter += 1;
    }

    if counter == 1 {
        return true;
    }

    let mut prev = head.unwrap();
    let mut next = prev.next.take();
    for _ in 0..(counter >> 1) - 1 {
        if let Some(mut node) = next {
            next = node.next.take();
            node.next = Some(prev);
            prev = node;
        }
    }

    let mut head_one = Some(prev);
    let mut head_two = if counter & 1 == 0 {
        next.as_ref()
    } else {
        next.as_ref().unwrap().next.as_ref()
    };

    while let Some(n1) = head_one {
        if let Some(n2) = head_two {
            if n1.val != n2.val {
                return false;
            }
            head_two = n2.next.as_ref();
        }
        head_one = n1.next;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = ListNode::from_vec(vec![1, 2, 3, 3, 2, 1]);
        assert!(is_palindrome(input));
    }

    #[test]
    fn example2() {
        let input = ListNode::from_vec(vec![1, 2]);
        assert!(!is_palindrome(input));
    }

    #[test]
    fn example3() {
        let input = ListNode::from_vec(vec![1, 2, 3, 1, 2, 3]);
        assert!(!is_palindrome(input));
    }
}
