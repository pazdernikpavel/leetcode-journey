use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type OptNode = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    pub fn max_depth(root: OptNode) -> i32 {
        Solution::track_depth(root, 0)
    }
    pub fn track_depth(node: OptNode, depth: i32) -> i32 {
        if let Some(unwrapped_node) = node {
            let mut borrowed_node = unwrapped_node.borrow_mut();
            max(
                Solution::track_depth(borrowed_node.left.take(), depth + 1),
                Solution::track_depth(borrowed_node.right.take(), depth + 1),
            )
        } else {
            depth
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test() {
        let input = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::max_depth(input), 3);
    }
}
